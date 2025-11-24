use std::collections::HashSet;

use mortar_compiler::{
    NodeDef, NodeJump, NodeStmt, Program, TopLevel,
    parser::{ChoiceDest, FunctionDecl},
};

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub nodes: Vec<String>,
    pub functions: Vec<FunctionDecl>,
    pub variables: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Analyze program and generate symbol table
pub fn analyze_program(program: &Program) -> Result<SymbolTable, Vec<(String, usize)>> {
    let mut symbol_table = SymbolTable::new();
    let mut errors = Vec::new();

    let mut node_names = HashSet::new();
    let mut function_names = HashSet::new();

    for item in &program.body {
        match item {
            TopLevel::NodeDef(node) => {
                if node_names.contains(&node.name) {
                    errors.push((format!("Node '{}' is defined repeatedly", node.name), 1));
                } else {
                    node_names.insert(node.name.clone());
                    symbol_table.nodes.push(node.name.clone());
                }

                // Verify node content
                analyze_node(node, &mut errors);
            }
            TopLevel::FunctionDecl(func) => {
                if function_names.contains(&func.name) {
                    errors.push((
                        format!("Duplicate definition of function '{}'", func.name),
                        1,
                    ));
                } else {
                    function_names.insert(func.name.clone());
                    symbol_table.functions.push(func.clone());
                }
            }
            TopLevel::VarDecl(_) | TopLevel::ConstDecl(_) | TopLevel::EnumDef(_) => {
                // Variable, constant, and enum declarations - no analysis needed yet
            }
            TopLevel::EventDef(_) | TopLevel::TimelineDef(_) => {
                // Event and timeline definitions - no analysis needed yet
            }
        }
    }

    // Verify whether the node jump target exists
    for item in &program.body {
        if let TopLevel::NodeDef(node) = item {
            validate_node_references(node, &node_names, &mut errors);
        }
    }

    if errors.is_empty() {
        Ok(symbol_table)
    } else {
        Err(errors)
    }
}

/// Analyze a single node
fn analyze_node(node: &NodeDef, errors: &mut Vec<(String, usize)>) {
    if node.body.is_empty() {
        errors.push((format!("Node '{}' cannot be empty", node.name), 1));
    }
    // TODO: Add more
}

/// Verify node reference
fn validate_node_references(
    node: &NodeDef,
    available_nodes: &HashSet<String>,
    errors: &mut Vec<(String, usize)>,
) {
    if let Some(jump) = &node.jump
        && let NodeJump::Identifier(target, _) = jump
        && !available_nodes.contains(target)
    {
        errors.push((
            format!(
                "Undefined node '{}' referenced in node '{}'",
                node.name, target
            ),
            1,
        ));
    }

    // Check node references in selection
    for stmt in &node.body {
        if let NodeStmt::Choice(choices) = stmt {
            for choice in choices {
                validate_choice_target(&choice.target, &node.name, available_nodes, errors);
            }
        }
    }
}

/// Validate selected target
fn validate_choice_target(
    target: &ChoiceDest,
    node_name: &str,
    available_nodes: &HashSet<String>,
    errors: &mut Vec<(String, usize)>,
) {
    match target {
        ChoiceDest::Identifier(target_name, _) => {
            if !available_nodes.contains(target_name) {
                errors.push((
                    format!(
                        "Undefined node '{}' is referenced in the selection of node '{}'",
                        node_name, target_name
                    ),
                    1,
                ));
            }
        }
        ChoiceDest::NestedChoices(nested) => {
            for choice in nested {
                validate_choice_target(&choice.target, node_name, available_nodes, errors);
            }
        }
        _ => {}
    }
}

/// Find symbol definition
pub fn find_symbol_at_position(
    symbol_table: &SymbolTable,
    symbol_name: &str,
) -> Option<SymbolInfo> {
    if symbol_table.nodes.iter().any(|n| n == symbol_name) {
        return Some(SymbolInfo {
            name: symbol_name.to_string(),
            kind: SymbolKind::Node,
            description: format!("Node: {}", symbol_name),
        });
    }

    if let Some(func) = symbol_table
        .functions
        .iter()
        .find(|f| f.name == symbol_name)
    {
        return Some(SymbolInfo {
            name: symbol_name.to_string(),
            kind: SymbolKind::Function,
            description: format!(
                "Function: {}({}){}",
                func.name,
                func.params
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.type_name))
                    .collect::<Vec<_>>()
                    .join(", "),
                func.return_type
                    .as_ref()
                    .map(|t| format!(" -> {}", t))
                    .unwrap_or_default()
            ),
        });
    }

    None
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Node,
    Function,
    Variable,
}
