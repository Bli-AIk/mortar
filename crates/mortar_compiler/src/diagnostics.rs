use crate::Language;
use crate::parser::{
    Arg, ChoiceDest, ChoiceItem, Condition, EventAction, FuncCall, FunctionDecl,
    InterpolatedString, NodeDef, NodeJump, NodeStmt, Program, StringPart, TimelineStmt, TopLevel,
};
use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet};

fn get_text(key: &str, language: Language) -> &'static str {
    match (key, language) {
        ("checking_file", Language::English) => "Checking file:",
        ("checking_file", Language::Chinese) => "检查文件:",
        ("error", Language::English) => "error",
        ("error", Language::Chinese) => "错误",
        ("warning", Language::English) => "warning",
        ("warning", Language::Chinese) => "警告",

        // Function naming warnings
        ("function_should_use_snake_case", Language::English) => {
            "Function '{}' should use snake_case naming."
        }
        ("function_should_use_snake_case", Language::Chinese) => {
            "函数 '{}' 应该使用 snake_case 命名。"
        }

        // Node naming warnings
        ("node_should_use_pascal_case", Language::English) => {
            "Node '{}' should use PascalCase naming."
        }
        ("node_should_use_pascal_case", Language::Chinese) => {
            "节点 '{}' 应该使用 PascalCase 命名。"
        }

        // Unused function warnings
        ("function_declared_but_never_used", Language::English) => {
            "Function '{}' is declared but never used."
        }
        ("function_declared_but_never_used", Language::Chinese) => "函数 '{}' 已声明但从未使用。",

        // Node not found errors
        ("node_not_defined", Language::English) => "Node '{}' is not defined.",
        ("node_not_defined", Language::Chinese) => "节点 '{}' 未定义",

        // Function not found errors
        ("function_not_declared", Language::English) => "Function '{}' is not declared.",
        ("function_not_declared", Language::Chinese) => "函数 '{}' 未声明。",

        // Argument count mismatch errors
        ("function_expects_args", Language::English) => {
            "Function '{}' expects {} arguments, but {} were provided."
        }
        ("function_expects_args", Language::Chinese) => {
            "函数 '{}' 期望 {} 个参数，但提供了 {} 个。"
        }

        // Argument type mismatch errors
        ("function_parameter_type_mismatch", Language::English) => {
            "Function '{}' parameter '{}' expects type '{}', but '{}' was provided."
        }
        ("function_parameter_type_mismatch", Language::Chinese) => {
            "函数 '{}' 的参数 '{}' 期望类型 '{}'，但提供了 '{}'。"
        }

        // Condition type mismatch errors
        ("condition_must_return_boolean", Language::English) => {
            "Condition function '{}' must return a boolean type, but returns '{}'."
        }
        ("condition_must_return_boolean", Language::Chinese) => {
            "条件函数 '{}' 必须返回布尔类型，但返回了 '{}'。"
        }

        _ => "",
    }
}

fn format_message(template: &str, args: &[&str]) -> String {
    let mut result = template.to_string();
    for arg in args {
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos + 2, arg);
        }
    }
    result
}

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

    pub fn print_diagnostics(&self, source: &str) {
        if self.diagnostics.is_empty() {
            return;
        }

        println!(
            "{} {}",
            get_text("checking_file", self.language),
            self.file_name.cyan()
        );

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
                Severity::Error => get_text("error", self.language),
                Severity::Warning => get_text("warning", self.language),
            };

            if let Some((start, _end)) = diagnostic.span {
                // Calculate line and column numbers
                let (line, col) = get_line_col(source, start);

                // Print colored error header
                let header = format!(
                    "{}: {}:{}:{}: {}",
                    severity_str, self.file_name, line, col, diagnostic.message
                );

                match diagnostic.severity {
                    Severity::Error => println!("{}", header.red()),
                    Severity::Warning => println!("{}", header.yellow()),
                }

                // Show source code snippet
                let lines: Vec<&str> = source.lines().collect();
                if line > 0 && (line - 1) < lines.len() {
                    let source_line = lines[line - 1];
                    println!(
                        "{:3} {} {}",
                        line.to_string().bright_blue(),
                        "|".bright_blue(),
                        source_line
                    );

                    // Calculate character count within error range for equal-length indicators
                    let error_length = if let Some((span_start, span_end)) = diagnostic.span {
                        // Convert byte length to character length
                        let error_text = &source[span_start..std::cmp::min(span_end, source.len())];
                        std::cmp::max(1, error_text.chars().count())
                    } else {
                        1
                    };

                    // Create pointer indicators with equal-length ^ characters
                    let padding = " ".repeat(col - 1); // col is 1-based, subtract 1 for correct position
                    let pointer_str = "^".repeat(error_length);
                    let pointer = match diagnostic.severity {
                        Severity::Error => {
                            format!("    {} {}{}", "|".bright_blue(), padding, pointer_str.red())
                        }
                        Severity::Warning => format!(
                            "    {} {}{}",
                            "|".bright_blue(),
                            padding,
                            pointer_str.yellow()
                        ),
                    };
                    println!("{}", pointer);
                }
            } else {
                let header = format!(
                    "{}: {}: {}",
                    severity_str, self.file_name, diagnostic.message
                );
                match diagnostic.severity {
                    Severity::Error => println!("{}", header.red()),
                    Severity::Warning => println!("{}", header.yellow()),
                }
            }
            println!();
        }
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
                    // Check function naming convention
                    if !is_snake_case(&func.name) {
                        self.add_diagnostic(Diagnostic {
                            kind: DiagnosticKind::NonSnakeCaseFunction {
                                function_name: func.name.clone(),
                            },
                            severity: Severity::Warning,
                            span: func.name_span,
                            message: format_message(
                                get_text("function_should_use_snake_case", self.language),
                                &[&func.name],
                            ),
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
                            message: format_message(
                                get_text("node_should_use_pascal_case", self.language),
                                &[&node.name],
                            ),
                        });
                    }

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
                    self.analyze_event_action(&event_def.action, &declared_functions, &mut used_functions);
                }
                TopLevel::TimelineDef(timeline_def) => {
                    for stmt in &timeline_def.body {
                        if let TimelineStmt::Run(run_stmt) = stmt {
                            // Mark the event/function as used
                            used_functions.insert(run_stmt.event_name.clone());
                        }
                    }
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
                NodeStmt::Run(_) => {
                    // Run statements don't need analysis for now
                }
                NodeStmt::WithEvents(_) => {
                    // WithEvents statements don't need analysis for now
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
                match condition {
                    Condition::Identifier(_) => {
                        // TODO: Check if identifier is valid in scope
                        // For now, we assume identifiers are boolean
                    }
                    Condition::FuncCall(func_call) => {
                        self.analyze_func_call(func_call, declared_functions, used_functions);

                        // Check that the function returns a boolean type
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
                            message: format_message(
                                get_text("node_not_defined", self.language),
                                &[node_name],
                            ),
                        });
                    }
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
            // Check argument types if count matches
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

        // Analyze nested function calls in arguments
        for arg in &func_call.args {
            if let Arg::FuncCall(nested_call) = arg {
                self.analyze_func_call(nested_call, declared_functions, used_functions);
            }
        }
    }

    fn infer_argument_type(
        &self,
        arg: &Arg,
        declared_functions: &HashMap<String, &FunctionDecl>,
    ) -> String {
        match arg {
            Arg::String(_) => "String".to_string(),
            Arg::Number(_) => "Number".to_string(),
            Arg::Boolean(_) => "Boolean".to_string(),
            Arg::Identifier(_) => "Unknown".to_string(), // Could be enhanced with variable tracking
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
        match (expected, actual) {
            // Exact matches
            ("String", "String") | ("Number", "Number") => true,
            // Boolean type aliases
            ("Boolean", "Bool")
            | ("Bool", "Boolean")
            | ("Boolean", "Boolean")
            | ("Bool", "Bool") => true,
            // Unknown types are compatible for now (could be enhanced)
            (_, "Unknown") => true,
            // Everything else is incompatible
            _ => false,
        }
    }

    fn analyze_interpolated_string(
        &mut self,
        interpolated: &InterpolatedString,
        declared_functions: &HashMap<String, &FunctionDecl>,
        used_functions: &mut HashSet<String>,
    ) {
        for part in &interpolated.parts {
            if let StringPart::Expression(func_call) = part {
                self.analyze_func_call(func_call, declared_functions, used_functions);

                // Also check that the function returns a type that can be converted to string
                // All types can be converted to string for interpolation, so no error needed
                // This is where we could add more specific checks if needed
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
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
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
    s.chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
}
