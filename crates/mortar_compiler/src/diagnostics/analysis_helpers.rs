//! # analysis_helpers.rs
//!
//! # analysis_helpers.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Contains the reusable semantic-analysis helpers used by the Mortar diagnostic
//! collector. It infers argument types, tracks function and timeline usage, validates naming
//! conventions, and checks choice conditions and function calls for type mismatches.
//!
//! 包含 Mortar 诊断收集器复用的语义分析辅助逻辑。它负责推断参数类型、跟踪函数和
//! 时间线的使用、验证命名规范，并检查选项条件和函数调用里的类型不匹配问题。

use std::collections::{HashMap, HashSet};

use crate::ast::{
    Arg, Condition, FuncCall, FunctionDecl, InterpolatedString, StringPart, TimelineDef,
    TimelineStmt,
};

use super::{Diagnostic, DiagnosticCollector, DiagnosticKind, Severity, format_message, get_text};

impl DiagnosticCollector {
    fn infer_argument_type(
        &self,
        arg: &Arg,
        declared_functions: &HashMap<String, &FunctionDecl>,
    ) -> String {
        match arg {
            Arg::String(_) => "String".to_string(),
            Arg::Number(_) => "Number".to_string(),
            Arg::Boolean(_) => "Boolean".to_string(),
            Arg::Identifier(_) => "Unknown".to_string(),
            Arg::FuncCall(func_call) => {
                if let Some(func_decl) = declared_functions.get(&func_call.name) {
                    func_decl
                        .return_type
                        .clone()
                        .unwrap_or("Unknown".to_string())
                } else {
                    "Unknown".to_string()
                }
            }
        }
    }

    fn is_boolean_type(&self, type_name: &str) -> bool {
        matches!(type_name, "Boolean" | "Bool")
    }

    fn is_type_compatible(&self, actual: &str, expected: &str) -> bool {
        matches!(
            (expected, actual),
            ("String", "String")
                | ("Number", "Number")
                | ("Boolean", "Bool")
                | ("Bool", "Boolean")
                | ("Boolean", "Boolean")
                | ("Bool", "Bool")
                | (_, "Unknown")
        )
    }

    pub(super) fn analyze_interpolated_string(
        &mut self,
        interpolated: &InterpolatedString,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        for part in &interpolated.parts {
            if let StringPart::Expression(func_call) = part {
                self.analyze_func_call(func_call, declared_functions, used_functions);
            }
        }
    }

    pub(super) fn analyze_text_interpolation(
        &mut self,
        text: &str,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch != '{' {
                continue;
            }
            let func_call = Self::collect_interpolation_expr(&mut chars);

            if let Some(func_name) = func_call
                .find('(')
                .map(|pos| func_call[..pos].trim().to_string())
                .filter(|name| declared_functions.contains_key(name))
            {
                used_functions.insert(func_name);
            }
        }
    }

    fn collect_interpolation_expr(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut expr = String::new();
        while let Some(&next_ch) = chars.peek() {
            if next_ch == '}' {
                chars.next();
                break;
            }
            expr.push(chars.next().expect("peeked char should exist"));
        }
        expr
    }

    pub(super) fn collect_timeline_usages(
        timeline_def: &TimelineDef,
        used_functions: &mut HashSet<String>,
    ) {
        for stmt in &timeline_def.body {
            if let TimelineStmt::Run(run_stmt) = stmt {
                used_functions.insert(run_stmt.event_name.clone());
            }
        }
    }

    pub(super) fn check_snake_case_naming(&mut self, name: &str, span: Option<(usize, usize)>) {
        if !is_snake_case(name) {
            self.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::NonSnakeCaseFunction {
                    function_name: name.to_string(),
                },
                severity: Severity::Warning,
                span,
                message: format_message(
                    get_text("function_should_use_snake_case", self.language),
                    &[name],
                ),
            });
        }
    }

    pub(super) fn check_pascal_case_naming(&mut self, name: &str, span: Option<(usize, usize)>) {
        if !is_pascal_case(name) {
            self.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::NonPascalCaseNode {
                    node_name: name.to_string(),
                },
                severity: Severity::Warning,
                span,
                message: format_message(
                    get_text("node_should_use_pascal_case", self.language),
                    &[name],
                ),
            });
        }
    }

    pub(super) fn analyze_choice_condition(
        &mut self,
        condition: &Condition,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        match condition {
            Condition::Identifier(_) => {}
            Condition::FuncCall(func_call) => {
                self.analyze_func_call(func_call, declared_functions, used_functions);

                if let Some(func_decl) = declared_functions.get(&func_call.name)
                    && let Some(return_type) = &func_decl.return_type
                    && !self.is_boolean_type(return_type)
                {
                    self.add_diagnostic(Diagnostic {
                        kind: DiagnosticKind::ConditionTypeMismatch {
                            expected: "Boolean".to_string(),
                            actual: return_type.clone(),
                        },
                        severity: Severity::Error,
                        span: func_call.name_span,
                        message: format_message(
                            get_text("condition_must_return_boolean", self.language),
                            &[&func_call.name, return_type],
                        ),
                    });
                }
            }
        }
    }

    pub(super) fn check_argument_types(
        &mut self,
        func_call: &FuncCall,
        func_decl: &FunctionDecl,
        declared_functions: &HashMap<String, &FunctionDecl>,
    ) {
        for (arg, param) in func_call.args.iter().zip(func_decl.params.iter()) {
            let arg_type = self.infer_argument_type(arg, declared_functions);
            if !self.is_type_compatible(&arg_type, &param.type_name) {
                let arg_type_for_message = self.infer_argument_type(arg, declared_functions);
                self.add_diagnostic(Diagnostic {
                    kind: DiagnosticKind::ArgumentTypeMismatch {
                        function_name: func_call.name.clone(),
                        parameter: param.name.clone(),
                        expected: param.type_name.clone(),
                        actual: arg_type,
                    },
                    severity: Severity::Error,
                    span: func_call.name_span,
                    message: format_message(
                        get_text("function_parameter_type_mismatch", self.language),
                        &[
                            &func_call.name,
                            &param.name,
                            &param.type_name,
                            &arg_type_for_message,
                        ],
                    ),
                });
            }
        }
    }
}

fn is_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let first_char = s.chars().next().expect("checked non-empty");
    if !first_char.is_ascii_lowercase() && first_char != '_' {
        return false;
    }

    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let first_char = s.chars().next().expect("checked non-empty");
    if !first_char.is_ascii_uppercase() {
        return false;
    }

    s.chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
}
