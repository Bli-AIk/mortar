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
        if let Some(document_entry) = self.documents.get(uri) {
            let rope = &document_entry.0;
            let line_idx = position.line as usize;

            let mut brace_depth = 0;
            let mut in_node = false;
            let mut in_function = false;

            for i in 0..=line_idx.min(rope.len_lines() - 1) {
                let line_content = rope.line(i).to_string();
                let trimmed = line_content.trim();

                if (trimmed.starts_with("node ") || trimmed.starts_with("nd ")) && brace_depth == 0
                {
                    in_node = false;
                }
                if (trimmed.starts_with("fn ") || trimmed.starts_with("function "))
                    && brace_depth == 0
                {
                    in_function = false;
                }

                for ch in line_content.chars() {
                    match ch {
                        '{' => {
                            brace_depth += 1;
                            if trimmed.starts_with("node ") || trimmed.starts_with("nd ") {
                                in_node = true;
                                in_function = false;
                            } else if trimmed.starts_with("fn ") || trimmed.starts_with("function ")
                            {
                                in_function = true;
                                in_node = false;
                            }
                        }
                        '}' => {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                in_node = false;
                                in_function = false;
                            }
                        }
                        _ => {}
                    }
                }

                if i == line_idx {
                    break;
                }
            }

            if line_idx < rope.len_lines() {
                let current_line = rope.line(line_idx).to_string();
                let current_trimmed = current_line.trim();
                if current_trimmed.contains("choice")
                    || current_trimmed.contains("->")
                    || (in_node
                        && (current_trimmed.starts_with("\"") || current_trimmed.contains("\"")))
                {
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
        } else {
            CompletionContext::TopLevel
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
            CompletionContext::TopLevel => {
                let mut completions = Vec::new();

                if "node".starts_with(current_word) {
                    completions.push(CompletionItem {
                        label: "node".to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        insert_text: Some("node ${1:node_name} {\n\t$0\n}".to_string()),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        detail: Some("Create a dialogue node".to_string()),
                        documentation: Some(Documentation::String(
                            "Create a new dialog node that defines dialog content and choices"
                                .to_string(),
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
                            "Create a new dialog node that defines dialog content and choices"
                                .to_string(),
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
            CompletionContext::InNode => {
                let mut completions = Vec::new();

                for keyword in &["text", "choice", "events"] {
                    if keyword.starts_with(current_word) {
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
                }

                completions
            }
            CompletionContext::InChoice => {
                let mut completions = Vec::new();

                for keyword in &["when", "return", "break"] {
                    if keyword.starts_with(current_word) {
                        let insert_text = match *keyword {
                            "when" => Some("when ${1:condition}".to_string()),
                            "return" => Some("return ${1:value}".to_string()),
                            _ => None,
                        };
                        let has_snippet = insert_text.is_some();
                        completions.push(CompletionItem {
                            label: keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            insert_text,
                            insert_text_format: if has_snippet {
                                Some(InsertTextFormat::SNIPPET)
                            } else {
                                None
                            },
                            ..CompletionItem::default()
                        });
                    }
                }

                // Add node completions
                for node in &symbol_table.nodes {
                    if node.starts_with(current_word) {
                        completions.push(CompletionItem {
                            label: node.clone(),
                            kind: Some(CompletionItemKind::CLASS),
                            detail: Some("Jump to node".to_string()),
                            ..CompletionItem::default()
                        });
                    }
                }

                completions
            }
            CompletionContext::InExpression => {
                let mut completions = Vec::new();

                // Add function completions
                for func in &symbol_table.functions {
                    if func.name.starts_with(current_word) {
                        completions.push(CompletionItem {
                            label: func.name.clone(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some(format!(
                                "fn {}({}){}",
                                func.name,
                                func.params
                                    .iter()
                                    .map(|p| format!("{}: {}", p.name, p.type_name))
                                    .collect::<Vec<_>>()
                                    .join(", "),
                                if func.return_type.as_ref().is_none_or(|t| t.is_empty()) {
                                    "".to_string()
                                } else {
                                    format!(" -> {}", func.return_type.as_ref().unwrap())
                                }
                            )),
                            ..CompletionItem::default()
                        });
                    }
                }

                // Add type keywords
                for type_keyword in &["String", "Number", "Boolean", "Bool", "true", "false"] {
                    if type_keyword.starts_with(current_word) {
                        completions.push(CompletionItem {
                            label: type_keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            ..CompletionItem::default()
                        });
                    }
                }

                completions
            }
        }
    }
}
