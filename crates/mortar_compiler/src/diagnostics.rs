//! # diagnostics.rs
//!
//! # diagnostics.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Provides the diagnostic system for reporting errors and warnings during compilation.
//!
//! 提供用于在编译期间报告错误和警告的诊断系统。
//!
//! It handles error collection, formatting, and semantic analysis checks (e.g., type checking, unused variables).
//!
//! 它处理错误收集、格式化和语义分析检查（例如类型检查、未使用的变量）。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! This file defines `Diagnostic`, `DiagnosticCollector`, and various analysis methods to validate the AST.
//!
//! 此文件定义了 `Diagnostic`、`DiagnosticCollector` 以及用于验证 AST 的各种分析方法。

use crate::Language;
use crate::ast::{
    Arg, ChoiceDest, ChoiceItem, EventAction, FuncCall, FunctionDecl, NodeDef, NodeJump, NodeStmt,
    Program, TopLevel,
};
use std::collections::{HashMap, HashSet};

mod analysis_helpers;
mod presentation;

use presentation::{format_message, get_text};

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    // Errors
    NodeNotFound {
        node_name: String,
    },
    FunctionNotFound {
        function_name: String,
    },
    SyntaxError {
        message: String,
    },
    TypeError {
        message: String,
    },
    ArgumentCountMismatch {
        function_name: String,
        expected: usize,
        actual: usize,
    },
    ArgumentTypeMismatch {
        function_name: String,
        parameter: String,
        expected: String,
        actual: String,
    },
    ConditionTypeMismatch {
        expected: String,
        actual: String,
    },

    // Warnings
    NonSnakeCaseFunction {
        function_name: String,
    },
    NonPascalCaseNode {
        node_name: String,
    },
    UnusedFunction {
        function_name: String,
    },
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
    language: Language,
}

impl DiagnosticCollector {
    pub fn new(file_name: String) -> Self {
        Self {
            diagnostics: Vec::new(),
            file_name,
            language: Language::English,
        }
    }

    pub fn new_with_language(file_name: String, language: Language) -> Self {
        Self {
            diagnostics: Vec::new(),
            file_name,
            language,
        }
    }

    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }

    pub fn get_diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
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
                    self.check_snake_case_naming(&func.name, func.name_span);
                    declared_functions.insert(func.name.clone(), func);
                }
                TopLevel::NodeDef(node) => {
                    self.check_pascal_case_naming(&node.name, node.name_span);
                    declared_nodes.insert(node.name.clone(), node);
                }
                TopLevel::VarDecl(_) | TopLevel::ConstDecl(_) | TopLevel::EnumDef(_) => {
                    // Variable, constant, and enum declarations don't need naming checks for now
                }
                TopLevel::EventDef(_) | TopLevel::TimelineDef(_) => {
                    // Event and timeline definitions don't need naming checks for now
                }
            }
        }

        // Second pass: check usages
        for item in &program.body {
            match item {
                TopLevel::NodeDef(node) => {
                    self.analyze_node_usage(
                        node,
                        &declared_functions,
                        &declared_nodes,
                        &mut used_functions,
                        &mut used_nodes,
                    );
                }
                TopLevel::EventDef(event_def) => {
                    self.analyze_event_action(
                        &event_def.action,
                        &declared_functions,
                        &mut used_functions,
                    );
                }
                TopLevel::TimelineDef(timeline_def) => {
                    Self::collect_timeline_usages(timeline_def, &mut used_functions)
                }
                TopLevel::VarDecl(var_decl) if var_decl.value.is_some() => {
                    let value = var_decl.value.as_ref().unwrap();
                    self.analyze_var_value(value, &declared_functions, &mut used_functions);
                }
                TopLevel::ConstDecl(const_decl) => {
                    self.analyze_var_value(
                        &const_decl.value,
                        &declared_functions,
                        &mut used_functions,
                    );
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
                    message: format_message(
                        get_text("function_declared_but_never_used", self.language),
                        &[func_name],
                    ),
                });
            }
        }
    }

    fn analyze_var_value(
        &mut self,
        value: &crate::ast::VarValue,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        let crate::ast::VarValue::Branch(branch_val) = value else {
            return;
        };
        for case in &branch_val.cases {
            let Some(events) = &case.events else { continue };
            for event in events {
                self.analyze_event_action(&event.action, declared_functions, used_functions);
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
                NodeStmt::IfElse(_) => {
                    // If-else statements don't need special analysis for now
                }
                NodeStmt::Branch(_) => {
                    // Branch definitions don't need analysis here
                }

                NodeStmt::Choice(choices) => {
                    self.analyze_choices(
                        choices,
                        declared_functions,
                        declared_nodes,
                        used_functions,
                        used_nodes,
                    );
                }
                NodeStmt::Text(text) => {
                    // Check for function calls in text interpolation (old format)
                    self.analyze_text_interpolation(text, declared_functions, used_functions);
                }
                NodeStmt::InterpolatedText(interpolated) => {
                    // Check function calls in interpolated string
                    self.analyze_interpolated_string(
                        interpolated,
                        declared_functions,
                        used_functions,
                    );
                }
                NodeStmt::Line(text) => {
                    self.analyze_text_interpolation(text, declared_functions, used_functions);
                }
                NodeStmt::InterpolatedLine(interpolated) => {
                    self.analyze_interpolated_string(
                        interpolated,
                        declared_functions,
                        used_functions,
                    );
                }
                NodeStmt::Run(_) => {
                    // Run statements don't need analysis for now
                }
                NodeStmt::WithEvents(with_events) => {
                    self.analyze_with_events(with_events, declared_functions, used_functions);
                }
                NodeStmt::VarDecl(_) => {
                    // Variable declarations in node body are local scope
                }
                NodeStmt::Assignment(_) => {
                    // Assignment statements don't need special analysis for now
                }
            }
        }
    }

    fn analyze_with_events(
        &mut self,
        with_events: &crate::ast::WithEventsStmt,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        for item in &with_events.events {
            self.analyze_with_event_item(item, declared_functions, used_functions);
        }
    }

    fn analyze_with_event_item(
        &mut self,
        item: &crate::ast::WithEventItem,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        match item {
            crate::ast::WithEventItem::InlineEvent(event) => {
                self.analyze_event_action(&event.action, declared_functions, used_functions);
            }
            crate::ast::WithEventItem::EventList(list) => {
                for sub_item in list {
                    self.analyze_with_event_item(sub_item, declared_functions, used_functions);
                }
            }
            crate::ast::WithEventItem::EventRef(_, _)
            | crate::ast::WithEventItem::EventRefWithOverride(_, _, _) => {
                // References point to TopLevel::EventDef, which are analyzed separately.
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
                        message: format_message(
                            get_text("node_not_defined", self.language),
                            &[node_name],
                        ),
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
                self.analyze_choice_condition(condition, declared_functions, used_functions);
            }

            // Analyze choice destination
            match &choice.target {
                ChoiceDest::Identifier(node_name, span) => {
                    self.check_node_exists(node_name, *span, declared_nodes, used_nodes);
                }
                ChoiceDest::NestedChoices(nested) => {
                    self.analyze_choices(
                        nested,
                        declared_functions,
                        declared_nodes,
                        used_functions,
                        used_nodes,
                    );
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
                message: format_message(
                    get_text("function_not_declared", self.language),
                    &[&func_call.name],
                ),
            });
            return;
        }

        let func_decl = declared_functions[&func_call.name];

        // Check argument count
        if func_call.args.len() != func_decl.params.len() {
            self.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::ArgumentCountMismatch {
                    function_name: func_call.name.clone(),
                    expected: func_decl.params.len(),
                    actual: func_call.args.len(),
                },
                severity: Severity::Error,
                span: func_call.name_span,
                message: format_message(
                    get_text("function_expects_args", self.language),
                    &[
                        &func_call.name,
                        &func_decl.params.len().to_string(),
                        &func_call.args.len().to_string(),
                    ],
                ),
            });
        } else {
            self.check_argument_types(func_call, func_decl, declared_functions);
        }

        // Analyze nested function calls in arguments
        for arg in &func_call.args {
            if let Arg::FuncCall(nested_call) = arg {
                self.analyze_func_call(nested_call, declared_functions, used_functions);
            }
        }
    }

    fn check_node_exists(
        &mut self,
        node_name: &str,
        span: Option<(usize, usize)>,
        declared_nodes: &HashMap<String, &NodeDef>,
        used_nodes: &mut HashSet<String>,
    ) {
        used_nodes.insert(node_name.to_string());
        if !declared_nodes.contains_key(node_name) {
            self.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::NodeNotFound {
                    node_name: node_name.to_string(),
                },
                severity: Severity::Error,
                span,
                message: format_message(get_text("node_not_defined", self.language), &[node_name]),
            });
        }
    }
}
