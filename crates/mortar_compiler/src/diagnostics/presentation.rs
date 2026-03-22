use owo_colors::OwoColorize;

use crate::Language;

use super::{Diagnostic, DiagnosticCollector, Severity};

pub(super) fn get_text(key: &str, language: Language) -> &'static str {
    match (key, language) {
        ("checking_file", Language::English) => "Checking file:",
        ("checking_file", Language::Chinese) => "检查文件:",
        ("error", Language::English) => "error",
        ("error", Language::Chinese) => "错误",
        ("warning", Language::English) => "warning",
        ("warning", Language::Chinese) => "警告",
        ("function_should_use_snake_case", Language::English) => {
            "Function '{}' should use snake_case naming."
        }
        ("function_should_use_snake_case", Language::Chinese) => {
            "函数 '{}' 应该使用 snake_case 命名。"
        }
        ("node_should_use_pascal_case", Language::English) => {
            "Node '{}' should use PascalCase naming."
        }
        ("node_should_use_pascal_case", Language::Chinese) => {
            "节点 '{}' 应该使用 PascalCase 命名。"
        }
        ("function_declared_but_never_used", Language::English) => {
            "Function '{}' is declared but never used."
        }
        ("function_declared_but_never_used", Language::Chinese) => "函数 '{}' 已声明但从未使用。",
        ("node_not_defined", Language::English) => "Node '{}' is not defined.",
        ("node_not_defined", Language::Chinese) => "节点 '{}' 未定义",
        ("function_not_declared", Language::English) => "Function '{}' is not declared.",
        ("function_not_declared", Language::Chinese) => "函数 '{}' 未声明。",
        ("function_expects_args", Language::English) => {
            "Function '{}' expects {} arguments, but {} were provided."
        }
        ("function_expects_args", Language::Chinese) => {
            "函数 '{}' 期望 {} 个参数，但提供了 {} 个。"
        }
        ("function_parameter_type_mismatch", Language::English) => {
            "Function '{}' parameter '{}' expects type '{}', but '{}' was provided."
        }
        ("function_parameter_type_mismatch", Language::Chinese) => {
            "函数 '{}' 的参数 '{}' 期望类型 '{}'，但提供了 '{}'。"
        }
        ("condition_must_return_boolean", Language::English) => {
            "Condition function '{}' must return a boolean type, but returns '{}'."
        }
        ("condition_must_return_boolean", Language::Chinese) => {
            "条件函数 '{}' 必须返回布尔类型，但返回了 '{}'。"
        }
        _ => "",
    }
}

pub(super) fn format_message(template: &str, args: &[&str]) -> String {
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

impl DiagnosticCollector {
    pub fn print_diagnostics(&self, source: &str) {
        if self.diagnostics.is_empty() {
            return;
        }

        println!(
            "{} {}",
            get_text("checking_file", self.language),
            self.file_name.cyan()
        );

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

            let Some((start, _end)) = diagnostic.span else {
                let header = format!(
                    "{}: {}: {}",
                    severity_str, self.file_name, diagnostic.message
                );
                match diagnostic.severity {
                    Severity::Error => println!("{}", header.red()),
                    Severity::Warning => println!("{}", header.yellow()),
                }
                println!();
                continue;
            };

            let (line, col) = get_line_col(source, start);
            let header = format!(
                "{}: {}:{}:{}: {}",
                severity_str, self.file_name, line, col, diagnostic.message
            );

            match diagnostic.severity {
                Severity::Error => println!("{}", header.red()),
                Severity::Warning => println!("{}", header.yellow()),
            }

            self.print_source_snippet(diagnostic, source, line, col);
            println!();
        }
    }

    fn print_source_snippet(&self, diagnostic: &Diagnostic, source: &str, line: usize, col: usize) {
        let lines: Vec<&str> = source.lines().collect();
        if line == 0 || (line - 1) >= lines.len() {
            return;
        }
        let source_line = lines[line - 1];
        println!(
            "{:3} {} {}",
            line.to_string().bright_blue(),
            "|".bright_blue(),
            source_line
        );

        let error_length = if let Some((span_start, span_end)) = diagnostic.span {
            let error_text = &source[span_start..std::cmp::min(span_end, source.len())];
            std::cmp::max(1, error_text.chars().count())
        } else {
            1
        };

        let padding = " ".repeat(col - 1);
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
}
