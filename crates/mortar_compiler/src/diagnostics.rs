use std::collections::{HashMap, HashSet};
use crate::parser::{Program, TopLevel, NodeDef, FunctionDecl, NodeStmt, FuncCall, Arg, ChoiceItem, ChoiceDest, Condition, EventAction, NodeJump};
use owo_colors::OwoColorize;

fn get_line_col(source: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    
    for (i, ch) in source.char_indices() {
        if i >= pos {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    
    (line, col)
}

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    // Errors
    NodeNotFound { node_name: String },
    FunctionNotFound { function_name: String },
    SyntaxError { message: String },
    TypeError { message: String },
    
    // Warnings
    NonSnakeCaseFunction { function_name: String },
    NonPascalCaseNode { node_name: String },
    UnusedFunction { function_name: String },
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub span: Option<(usize, usize)>, // (start, end) byte positions
    pub message: String,
}

pub struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
    file_name: String,
}

impl DiagnosticCollector {
    pub fn new(file_name: String) -> Self {
        Self {
            diagnostics: Vec::new(),
            file_name,
        }
    }

    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.severity, Severity::Error))
    }

    pub fn print_diagnostics(&self, source: &str) {
        if self.diagnostics.is_empty() {
            return;
        }

        println!("Checking file: {}", self.file_name.cyan());
        
        // Sort diagnostics: errors first, then warnings
        let mut sorted_diagnostics = self.diagnostics.clone();
        sorted_diagnostics.sort_by(|a, b| {
            use Severity::*;
            match (&a.severity, &b.severity) {
                (Error, Warning) => std::cmp::Ordering::Less,
                (Warning, Error) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });
        
        for diagnostic in &sorted_diagnostics {
            let severity_str = match diagnostic.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
            };

            if let Some((start, _end)) = diagnostic.span {
                // 计算行号和列号
                let (line, col) = get_line_col(source, start);
                
                // 打印错误信息头部，使用彩色
                let header = format!(
                    "{}: {}:{}:{}: {}",
                    severity_str,
                    self.file_name,
                    line,
                    col,
                    diagnostic.message
                );
                
                match severity_str {
                    "error" => println!("{}", header.red()),
                    "warning" => println!("{}", header.yellow()),
                    _ => println!("{}", header),
                }
                
                // 显示源代码片段
                let lines: Vec<&str> = source.lines().collect();
                if line > 0 && (line - 1) < lines.len() {
                    let source_line = lines[line - 1];
                    println!("{:3} {} {}", line.to_string().bright_blue(), "|".bright_blue(), source_line);
                    
                    // 计算错误范围内的字符数量来生成等长的指示符
                    let error_length = if let Some((span_start, span_end)) = diagnostic.span {
                        // 直接使用字节长度，转换为字符长度
                        let error_text = &source[span_start..std::cmp::min(span_end, source.len())];
                        std::cmp::max(1, error_text.chars().count())
                    } else {
                        1
                    };
                    
                    // 创建指示符，使用等长的 ^ 字符
                    let padding = " ".repeat(col - 1); // col已经是1-based，减1得到正确的位置
                    let pointer_str = "^".repeat(error_length);
                    let pointer = match severity_str {
                        "error" => format!("    {} {}{}", "|".bright_blue(), padding, pointer_str.red()),
                        "warning" => format!("    {} {}{}", "|".bright_blue(), padding, pointer_str.yellow()),
                        _ => format!("    {} {}{}", "|".bright_blue(), padding, pointer_str),
                    };
                    println!("{}", pointer);
                }
            } else {
                let header = format!("{}: {}: {}", severity_str, self.file_name, diagnostic.message);
                match severity_str {
                    "error" => println!("{}", header.red()),
                    "warning" => println!("{}", header.yellow()),
                    _ => println!("{}", header),
                }
            }
            println!();
        }
    }

    pub fn analyze_program(&mut self, program: &Program) {
        // Collect all function declarations and nodes
        let mut declared_functions = HashMap::new();
        let mut declared_nodes = HashMap::new();
        let mut used_functions = HashSet::new();
        let mut used_nodes = HashSet::new();

        // First pass: collect declarations
        for item in &program.body {
            match item {
                TopLevel::FunctionDecl(func) => {
                    // Check function naming convention
                    if !is_snake_case(&func.name) {
                        self.add_diagnostic(Diagnostic {
                            kind: DiagnosticKind::NonSnakeCaseFunction {
                                function_name: func.name.clone(),
                            },
                            severity: Severity::Warning,
                            span: func.name_span,
                            message: format!("Function '{}' should use snake_case naming", func.name),
                        });
                    }

                    declared_functions.insert(func.name.clone(), func);
                }
                TopLevel::NodeDef(node) => {
                    // Check node naming convention
                    if !is_pascal_case(&node.name) {
                        self.add_diagnostic(Diagnostic {
                            kind: DiagnosticKind::NonPascalCaseNode {
                                node_name: node.name.clone(),
                            },
                            severity: Severity::Warning,
                            span: node.name_span,
                            message: format!("Node '{}' should use PascalCase naming", node.name),
                        });
                    }

                    declared_nodes.insert(node.name.clone(), node);
                }
            }
        }

        // Second pass: check usages
        for item in &program.body {
            match item {
                TopLevel::NodeDef(node) => {
                    self.analyze_node_usage(node, &declared_functions, &declared_nodes, 
                                           &mut used_functions, &mut used_nodes);
                }
                _ => {}
            }
        }

        // Check for unused functions
        for func_name in declared_functions.keys() {
            if !used_functions.contains(func_name) {
                self.add_diagnostic(Diagnostic {
                    kind: DiagnosticKind::UnusedFunction {
                        function_name: func_name.clone(),
                    },
                    severity: Severity::Warning,
                    span: declared_functions[func_name].name_span,
                    message: format!("Function '{}' is declared but never used", func_name),
                });
            }
        }
    }

    fn analyze_node_usage(
        &mut self,
        node: &NodeDef,
        declared_functions: &HashMap<String, &FunctionDecl>,
        declared_nodes: &HashMap<String, &NodeDef>,
        used_functions: &mut HashSet<String>,
        used_nodes: &mut HashSet<String>,
    ) {
        // Analyze node jump
        if let Some(jump) = &node.jump {
            self.analyze_node_jump(jump, declared_nodes, used_nodes);
        }

        // Analyze statements
        for stmt in &node.body {
            match stmt {
                NodeStmt::Events(events) => {
                    for event in events {
                        self.analyze_event_action(&event.action, declared_functions, used_functions);
                    }
                }
                NodeStmt::Choice(choices) => {
                    self.analyze_choices(choices, declared_functions, declared_nodes, 
                                       used_functions, used_nodes);
                }
                NodeStmt::Text(text) => {
                    // Check for function calls in text interpolation
                    self.analyze_text_interpolation(text, declared_functions, used_functions);
                }
            }
        }
    }

    fn analyze_node_jump(
        &mut self,
        jump: &NodeJump,
        declared_nodes: &HashMap<String, &NodeDef>,
        used_nodes: &mut HashSet<String>,
    ) {
        match jump {
            NodeJump::Identifier(node_name, span) => {
                used_nodes.insert(node_name.clone());
                if !declared_nodes.contains_key(node_name) {
                    self.add_diagnostic(Diagnostic {
                        kind: DiagnosticKind::NodeNotFound {
                            node_name: node_name.clone(),
                        },
                        severity: Severity::Error,
                        span: *span,
                        message: format!("Node '{}' is not defined", node_name),
                    });
                }
            }
            NodeJump::Return | NodeJump::Break => {
                // These are always valid
            }
        }
    }

    fn analyze_choices(
        &mut self,
        choices: &[ChoiceItem],
        declared_functions: &HashMap<String, &FunctionDecl>,
        declared_nodes: &HashMap<String, &NodeDef>,
        used_functions: &mut HashSet<String>,
        used_nodes: &mut HashSet<String>,
    ) {
        for choice in choices {
            // Analyze condition
            if let Some(condition) = &choice.condition {
                match condition {
                    Condition::Identifier(_) => {
                        // TODO: Check if identifier is valid in scope
                    }
                    Condition::FuncCall(func_call) => {
                        self.analyze_func_call(func_call, declared_functions, used_functions);
                    }
                }
            }

            // Analyze choice destination
            match &choice.target {
                ChoiceDest::Identifier(node_name, span) => {
                    used_nodes.insert(node_name.clone());
                    if !declared_nodes.contains_key(node_name) {
                        self.add_diagnostic(Diagnostic {
                            kind: DiagnosticKind::NodeNotFound {
                                node_name: node_name.clone(),
                            },
                            severity: Severity::Error,
                            span: *span,
                            message: format!("Node '{}' is not defined", node_name),
                        });
                    }
                }
                ChoiceDest::NestedChoices(nested) => {
                    self.analyze_choices(nested, declared_functions, declared_nodes, 
                                       used_functions, used_nodes);
                }
                ChoiceDest::Return | ChoiceDest::Break => {
                    // These are always valid
                }
            }
        }
    }

    fn analyze_event_action(
        &mut self,
        action: &EventAction,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        self.analyze_func_call(&action.call, declared_functions, used_functions);
        
        for chain in &action.chains {
            self.analyze_func_call(chain, declared_functions, used_functions);
        }
    }

    fn analyze_func_call(
        &mut self,
        func_call: &FuncCall,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        used_functions.insert(func_call.name.clone());

        // Check if function is declared
        if !declared_functions.contains_key(&func_call.name) {
            self.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::FunctionNotFound {
                    function_name: func_call.name.clone(),
                },
                severity: Severity::Error,
                span: func_call.name_span,
                message: format!("Function '{}' is not declared", func_call.name),
            });
            return;
        }

        let func_decl = declared_functions[&func_call.name];

        // Check argument count and types
        if func_call.args.len() != func_decl.params.len() {
            self.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::TypeError {
                    message: format!(
                        "Function '{}' expects {} arguments, but {} were provided",
                        func_call.name,
                        func_decl.params.len(),
                        func_call.args.len()
                    ),
                },
                severity: Severity::Error,
                span: func_call.name_span,
                message: format!(
                    "Function '{}' expects {} arguments, but {} were provided",
                    func_call.name,
                    func_decl.params.len(),
                    func_call.args.len()
                ),
            });
        }

        // Analyze nested function calls in arguments
        for arg in &func_call.args {
            if let Arg::FuncCall(nested_call) = arg {
                self.analyze_func_call(nested_call, declared_functions, used_functions);
            }
        }
    }

    fn analyze_text_interpolation(
        &mut self,
        text: &str,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        // Simple regex-like parsing to find function calls in {function_name()} pattern
        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Found potential function call
                let mut func_call = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '}' {
                        chars.next(); // consume '}'
                        break;
                    }
                    func_call.push(chars.next().unwrap());
                }
                
                // Check if it looks like a function call
                if let Some(paren_pos) = func_call.find('(') {
                    let func_name = func_call[..paren_pos].trim().to_string();
                    if declared_functions.contains_key(&func_name) {
                        used_functions.insert(func_name);
                    }
                }
            }
        }
    }
}

fn is_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Should start with lowercase letter or underscore
    let first_char = s.chars().next().unwrap();
    if !first_char.is_ascii_lowercase() && first_char != '_' {
        return false;
    }

    // Should contain only lowercase letters, digits, and underscores
    s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Should start with uppercase letter
    let first_char = s.chars().next().unwrap();
    if !first_char.is_ascii_uppercase() {
        return false;
    }

    // Should contain only letters and digits
    s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case() {
        assert!(is_snake_case("hello"));
        assert!(is_snake_case("hello_world"));
        assert!(is_snake_case("get_name"));
        assert!(is_snake_case("_private"));
        
        assert!(!is_snake_case("Hello"));
        assert!(!is_snake_case("HelloWorld"));
        assert!(!is_snake_case("hello-world"));
        assert!(!is_snake_case(""));
    }

    #[test]
    fn test_pascal_case() {
        assert!(is_pascal_case("Hello"));
        assert!(is_pascal_case("HelloWorld"));
        assert!(is_pascal_case("Start"));
        assert!(is_pascal_case("ChoicePoint"));
        
        assert!(!is_pascal_case("hello"));
        assert!(!is_pascal_case("helloWorld"));
        assert!(!is_pascal_case("hello_world"));
        assert!(!is_pascal_case(""));
    }
}