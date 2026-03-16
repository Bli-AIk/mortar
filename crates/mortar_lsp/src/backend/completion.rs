use tower_lsp_server::lsp_types::*;

use crate::analysis::SymbolTable;
use crate::backend::Backend;

/// Autocomplete context type
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionContext {
    TopLevel,
    InNode,
    InChoice,
    InExpression,
}

impl Backend {
    /// Context analysis based on entire document
    pub fn analyze_document_context(&self, uri: &Uri, position: Position) -> CompletionContext {
        let Some(document_entry) = self.documents.get(uri) else {
            return CompletionContext::TopLevel;
        };

        let rope = &document_entry.0;
        let line_idx = position.line as usize;

        let mut brace_depth = 0;
        let mut in_node = false;
        let mut in_function = false;

        for i in 0..=line_idx.min(rope.len_lines() - 1) {
            let line_content = rope.line(i).to_string();
            let trimmed = line_content.trim();

            let is_node_decl = trimmed.starts_with("node ") || trimmed.starts_with("nd ");
            let is_fn_decl = trimmed.starts_with("fn ") || trimmed.starts_with("function ");

            if is_node_decl && brace_depth == 0 {
                in_node = false;
            }
            if is_fn_decl && brace_depth == 0 {
                in_function = false;
            }

            Self::update_brace_state(
                &line_content,
                is_node_decl,
                is_fn_decl,
                &mut brace_depth,
                &mut in_node,
                &mut in_function,
            );

            if i == line_idx {
                break;
            }
        }

        if line_idx < rope.len_lines() {
            let current_line = rope.line(line_idx).to_string();
            let current_trimmed = current_line.trim();
            let is_choice = current_trimmed.contains("choice")
                || current_trimmed.contains("->")
                || (in_node && current_trimmed.contains("\""));
            if is_choice {
                return CompletionContext::InChoice;
            }
        }

        if in_function && brace_depth > 0 {
            CompletionContext::InExpression
        } else if in_node && brace_depth > 0 {
            CompletionContext::InNode
        } else {
            CompletionContext::TopLevel
        }
    }

    fn update_brace_state(
        line_content: &str,
        is_node_decl: bool,
        is_fn_decl: bool,
        brace_depth: &mut i32,
        in_node: &mut bool,
        in_function: &mut bool,
    ) {
        let open_count = line_content.chars().filter(|&c| c == '{').count() as i32;
        let close_count = line_content.chars().filter(|&c| c == '}').count() as i32;

        if open_count > 0 {
            *brace_depth += open_count;
            if is_node_decl {
                *in_node = true;
                *in_function = false;
            } else if is_fn_decl {
                *in_function = true;
                *in_node = false;
            }
        }

        if close_count > 0 {
            *brace_depth -= close_count;
            if *brace_depth == 0 {
                *in_node = false;
                *in_function = false;
            }
        }
    }

    /// Analyze the context of auto-completion
    pub fn analyze_completion_context(&self, line: &str, _char_idx: usize) -> CompletionContext {
        let trimmed = line.trim();

        if trimmed.is_empty() || (!trimmed.contains('{') && !trimmed.contains('}')) {
            return CompletionContext::TopLevel;
        }

        if trimmed.starts_with("node ")
            || trimmed.starts_with("nd ")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("function ")
        {
            return if trimmed.ends_with('{') {
                CompletionContext::InNode
            } else {
                CompletionContext::TopLevel
            };
        }

        if trimmed.contains("choice:") || trimmed.contains("->") {
            return CompletionContext::InChoice;
        }

        if trimmed.starts_with("text:") || trimmed.starts_with("events:") {
            return CompletionContext::InNode;
        }

        CompletionContext::TopLevel
    }

    pub fn generate_completion_items(&self, context: CompletionContext) -> Vec<CompletionItem> {
        match context {
            CompletionContext::TopLevel => vec![
                CompletionItem {
                    label: "node".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define a new story node".to_string()),
                    insert_text: Some("node ${1:node_name} {\n    text: \"${2:text_content}\"\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "nd".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define a new story node (short form)".to_string()),
                    insert_text: Some("nd ${1:node_name} {\n    text: \"${2:text_content}\"\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "fn".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define a function".to_string()),
                    insert_text: Some("fn ${1:function_name}(${2:params}) -> ${3:ReturnType} {\n    ${4:// function body}\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
            ],
            CompletionContext::InNode => vec![
                CompletionItem {
                    label: "text".to_string(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some("Story text content".to_string()),
                    insert_text: Some("text: \"${1:text_content}\"".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "choice".to_string(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some("Player choices".to_string()),
                    insert_text: Some("choice: [\n    \"${1:choice_text}\" -> ${2:target_node}\n]".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "events".to_string(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some("Timed events".to_string()),
                    insert_text: Some("events: [\n    ${1:delay}, ${2:action}\n]".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
            ],
            CompletionContext::InChoice => vec![
                CompletionItem {
                    label: "when".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Conditional choice".to_string()),
                    insert_text: Some("when ${1:condition}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "->".to_string(),
                    kind: Some(CompletionItemKind::OPERATOR),
                    detail: Some("Jump to target node".to_string()),
                    insert_text: Some("-> ${1:target_node}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
            ],
            CompletionContext::InExpression => vec![
                CompletionItem {
                    label: "return".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Return statement".to_string()),
                    insert_text: Some("return ${1:value}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "break".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Break statement".to_string()),
                    ..Default::default()
                },
            ],
        }
    }

    /// Generate filtered completion items based on current word and symbol table
    pub fn generate_completion_items_filtered(
        &self,
        context: CompletionContext,
        current_word: &str,
        symbol_table: &SymbolTable,
    ) -> Vec<CompletionItem> {
        match context {
            CompletionContext::TopLevel => Self::completions_top_level(current_word),
            CompletionContext::InNode => Self::completions_in_node(current_word),
            CompletionContext::InChoice => Self::completions_in_choice(current_word, symbol_table),
            CompletionContext::InExpression => {
                Self::completions_in_expression(current_word, symbol_table)
            }
        }
    }

    fn completions_top_level(current_word: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        if "node".starts_with(current_word) {
            completions.push(CompletionItem {
                label: "node".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("node ${1:node_name} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a dialogue node".to_string()),
                documentation: Some(Documentation::String(
                    "Create a new dialog node that defines dialog content and choices".to_string(),
                )),
                ..CompletionItem::default()
            });
        }

        if "nd".starts_with(current_word) {
            completions.push(CompletionItem {
                label: "nd".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("nd ${1:node_name} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a dialogue node".to_string()),
                documentation: Some(Documentation::String(
                    "Create a new dialog node that defines dialog content and choices".to_string(),
                )),
                ..CompletionItem::default()
            });
        }

        if "fn".starts_with(current_word) {
            completions.push(CompletionItem {
                label: "fn".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("fn ${1:function_name}()".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a function".to_string()),
                documentation: Some(Documentation::String(
                    "Create a new function definition".to_string(),
                )),
                ..CompletionItem::default()
            });
        }

        if "function".starts_with(current_word) {
            completions.push(CompletionItem {
                label: "function".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("function ${1:function_name}() {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a function".to_string()),
                documentation: Some(Documentation::String(
                    "Create a new function definition (alternative syntax)".to_string(),
                )),
                ..CompletionItem::default()
            });
        }

        completions
    }

    fn completions_in_node(current_word: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        for keyword in &["text", "choice", "events"] {
            if !keyword.starts_with(current_word) {
                continue;
            }
            let (insert_text, detail) = match *keyword {
                "text" => (
                    "text: \"${1:text_content}\"".to_string(),
                    "Story text content".to_string(),
                ),
                "choice" => (
                    "choice: [\n\t\"${1:text}\" -> ${2:target_node}\n]".to_string(),
                    "Add selection".to_string(),
                ),
                "events" => (
                    "events: [\n\t${1:delay}, ${2:action}\n]".to_string(),
                    "Timed events".to_string(),
                ),
                _ => (keyword.to_string(), "".to_string()),
            };
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                insert_text: Some(insert_text),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some(detail),
                ..CompletionItem::default()
            });
        }

        completions
    }

    fn completions_in_choice(
        current_word: &str,
        symbol_table: &SymbolTable,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        for keyword in &["when", "return", "break"] {
            if !keyword.starts_with(current_word) {
                continue;
            }
            let insert_text = match *keyword {
                "when" => Some("when ${1:condition}".to_string()),
                "return" => Some("return ${1:value}".to_string()),
                _ => None,
            };
            let insert_text_format = insert_text.as_ref().map(|_| InsertTextFormat::SNIPPET);
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text,
                insert_text_format,
                ..CompletionItem::default()
            });
        }

        // Add node completions
        for node in &symbol_table.nodes {
            if !node.starts_with(current_word) {
                continue;
            }
            completions.push(CompletionItem {
                label: node.clone(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("Jump to node".to_string()),
                ..CompletionItem::default()
            });
        }

        completions
    }

    fn completions_in_expression(
        current_word: &str,
        symbol_table: &SymbolTable,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Add function completions
        for func in &symbol_table.functions {
            if !func.name.starts_with(current_word) {
                continue;
            }
            let return_suffix = match func.return_type.as_ref().filter(|t| !t.is_empty()) {
                Some(rt) => format!(" -> {rt}"),
                None => String::new(),
            };
            let params_str = func
                .params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.type_name))
                .collect::<Vec<_>>()
                .join(", ");
            completions.push(CompletionItem {
                label: func.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!("fn {}({}){}", func.name, params_str, return_suffix)),
                ..CompletionItem::default()
            });
        }

        // Add type keywords
        for type_keyword in &["String", "Number", "Boolean", "Bool", "true", "false"] {
            if !type_keyword.starts_with(current_word) {
                continue;
            }
            completions.push(CompletionItem {
                label: type_keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            });
        }

        completions
    }
}
