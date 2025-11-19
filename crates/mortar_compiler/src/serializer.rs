use crate::Language;
use crate::parser::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

fn get_text(key: &str, language: Language) -> &'static str {
    match (key, language) {
        ("generated", Language::English) => "Generated:",
        ("generated", Language::Chinese) => "生成文件:",
        _ => "",
    }
}

#[derive(Serialize, Deserialize)]
pub struct MortaredOutput {
    metadata: Metadata,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    variables: Vec<JsonVariable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    constants: Vec<JsonConstant>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    enums: Vec<JsonEnum>,
    nodes: Vec<JsonNode>,
    functions: Vec<JsonFunction>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    version: String,
    generated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct JsonNode {
    name: String,
    texts: Vec<JsonText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<JsonBranchDef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choice: Option<Vec<JsonChoice>>,
}

#[derive(Serialize, Deserialize)]
struct JsonBranchDef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_type: Option<String>,
    cases: Vec<JsonBranchCase>,
}

#[derive(Serialize, Deserialize)]
struct JsonText {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpolated_parts: Option<Vec<JsonStringPart>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<JsonEvent>>,
}

#[derive(Serialize, Deserialize)]
struct JsonStringPart {
    #[serde(rename = "type")]
    part_type: String, // "text", "expression", or "branch"
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<JsonBranchCase>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct JsonBranchCase {
    condition: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<JsonEvent>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct JsonEvent {
    index: f64,
    actions: Vec<JsonAction>,
}

#[derive(Serialize, Deserialize, Clone)]
struct JsonAction {
    #[serde(rename = "type")]
    action_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonChoice {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<JsonCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choice: Option<Vec<JsonChoice>>,
}

#[derive(Serialize, Deserialize)]
struct JsonCondition {
    #[serde(rename = "type")]
    condition_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonFunction {
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<JsonParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "return")]
    return_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonParam {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[derive(Serialize, Deserialize)]
struct JsonVariable {
    name: String,
    #[serde(rename = "type")]
    var_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct JsonConstant {
    name: String,
    #[serde(rename = "type")]
    const_type: String,
    value: serde_json::Value,
    public: bool,
}

#[derive(Serialize, Deserialize)]
struct JsonEnum {
    name: String,
    variants: Vec<String>,
}

pub struct Serializer;

impl Serializer {
    pub fn serialize_to_json(program: &Program, pretty: bool) -> Result<String, String> {
        let mortared = Self::convert_program_to_mortared(program)?;
        if pretty {
            serde_json::to_string_pretty(&mortared)
                .map_err(|e| format!("Serialization error: {}", e))
        } else {
            serde_json::to_string(&mortared).map_err(|e| format!("Serialization error: {}", e))
        }
    }

    pub fn save_to_file(program: &Program, input_path: &str, pretty: bool) -> Result<(), String> {
        Self::save_to_file_with_language(program, input_path, pretty, Language::English)
    }

    pub fn save_to_file_with_language(
        program: &Program,
        input_path: &str,
        pretty: bool,
        language: Language,
    ) -> Result<(), String> {
        let input_path = Path::new(input_path);
        let json_content = Self::serialize_to_json(program, pretty)?;

        let output_path = input_path.with_extension("mortared");
        std::fs::write(&output_path, json_content)
            .map_err(|e| format!("Failed to write file {}: {}", output_path.display(), e))?;

        println!(
            "{} {}",
            get_text("generated", language),
            output_path.display()
        );
        Ok(())
    }

    fn convert_program_to_mortared(program: &Program) -> Result<MortaredOutput, String> {
        let metadata = Metadata {
            version: "0.1.0".to_string(),
            generated_at: Utc::now(),
        };

        let mut variables = Vec::new();
        let mut constants = Vec::new();
        let mut enums = Vec::new();
        let mut nodes = Vec::new();
        let mut functions = Vec::new();

        for top_level in &program.body {
            match top_level {
                TopLevel::NodeDef(node_def) => {
                    nodes.push(Self::convert_node_def(node_def)?);
                }
                TopLevel::FunctionDecl(func_decl) => {
                    functions.push(Self::convert_function_decl(func_decl));
                }
                TopLevel::VarDecl(var_decl) => {
                    variables.push(Self::convert_var_decl(var_decl));
                }
                TopLevel::ConstDecl(const_decl) => {
                    constants.push(Self::convert_const_decl(const_decl));
                }
                TopLevel::EnumDef(enum_def) => {
                    enums.push(Self::convert_enum_def(enum_def));
                }
            }
        }

        Ok(MortaredOutput {
            metadata,
            variables,
            constants,
            enums,
            nodes,
            functions,
        })
    }

    fn convert_node_def(node_def: &NodeDef) -> Result<JsonNode, String> {
        let mut texts = Vec::new();
        let mut choices = None;
        let mut branches_vec = Vec::new();

        // Group texts and events, separate choices and branches
        let mut current_text: Option<String> = None;
        let mut current_events: Vec<JsonEvent> = Vec::new();

        for stmt in &node_def.body {
            match stmt {
                NodeStmt::Branch(branch_def) => {
                    // Collect branch definitions
                    branches_vec.push(Self::convert_branch_def(branch_def)?);
                    continue;
                }
                NodeStmt::Text(text) => {
                    // If we have a current text, save it first
                    if let Some(text_content) = current_text.take() {
                        texts.push(JsonText {
                            text: text_content,
                            interpolated_parts: None,
                            events: if current_events.is_empty() {
                                None
                            } else {
                                Some(current_events.clone())
                            },
                        });
                        current_events.clear();
                    }
                    current_text = Some(text.clone());
                }
                NodeStmt::InterpolatedText(interpolated) => {
                    // If we have a current text, save it first
                    if let Some(text_content) = current_text.take() {
                        texts.push(JsonText {
                            text: text_content,
                            interpolated_parts: None,
                            events: if current_events.is_empty() {
                                None
                            } else {
                                Some(current_events.clone())
                            },
                        });
                        current_events.clear();
                    }

                    // Convert interpolated string
                    let (rendered_text, parts) = Self::convert_interpolated_string(interpolated)?;
                    texts.push(JsonText {
                        text: rendered_text,
                        interpolated_parts: Some(parts),
                        events: if current_events.is_empty() {
                            None
                        } else {
                            Some(current_events.clone())
                        },
                    });
                    current_events.clear();
                }
                NodeStmt::Events(events) => {
                    // Convert events and associate with current text
                    for event in events {
                        current_events.push(Self::convert_event(event)?);
                    }
                }
                NodeStmt::Choice(choice_items) => {
                    // Save any pending text first
                    if let Some(text_content) = current_text.take() {
                        texts.push(JsonText {
                            text: text_content,
                            interpolated_parts: None,
                            events: if current_events.is_empty() {
                                None
                            } else {
                                Some(current_events.clone())
                            },
                        });
                        current_events.clear();
                    }

                    let mut json_choices = Vec::new();
                    for item in choice_items {
                        json_choices.push(Self::convert_choice_item(item)?);
                    }
                    choices = Some(json_choices);
                }
            }
        }

        // Don't forget the last text if any
        if let Some(text_content) = current_text {
            texts.push(JsonText {
                text: text_content,
                interpolated_parts: None,
                events: if current_events.is_empty() {
                    None
                } else {
                    Some(current_events)
                },
            });
        }

        let next = match &node_def.jump {
            Some(NodeJump::Identifier(name, _)) => Some(name.clone()),
            _ => None,
        };

        Ok(JsonNode {
            name: node_def.name.clone(),
            texts,
            branches: if branches_vec.is_empty() { None } else { Some(branches_vec) },
            next,
            choice: choices,
        })
    }
    
    fn convert_branch_def(branch_def: &BranchDef) -> Result<JsonBranchDef, String> {
        let cases = branch_def.cases
            .iter()
            .map(|case| {
                let events = if let Some(event_list) = &case.events {
                    Some(event_list
                        .iter()
                        .map(|e| Self::convert_event(e))
                        .collect::<Result<Vec<_>, _>>()?)
                } else {
                    None
                };
                
                Ok(JsonBranchCase {
                    condition: case.condition.clone(),
                    text: case.text.clone(),
                    events,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        
        Ok(JsonBranchDef {
            name: branch_def.name.clone(),
            enum_type: branch_def.enum_type.clone(),
            cases,
        })
    }

    fn convert_event(event: &Event) -> Result<JsonEvent, String> {
        let mut actions = vec![Self::convert_func_call_to_action(&event.action.call)?];

        // Add chained actions
        for chain_call in &event.action.chains {
            actions.push(Self::convert_func_call_to_action(chain_call)?);
        }

        Ok(JsonEvent {
            index: event.index,
            actions,
        })
    }

    fn convert_func_call_to_action(func_call: &FuncCall) -> Result<JsonAction, String> {
        let mut args = Vec::new();

        for arg in &func_call.args {
            match arg {
                Arg::String(s) => args.push(s.clone()),
                Arg::Number(n) => args.push(n.to_string()),
                Arg::Boolean(b) => args.push(b.to_string()),
                Arg::Identifier(id) => args.push(id.clone()),
                Arg::FuncCall(_) => {
                    return Err(
                        "Nested function calls in arguments not supported in JSON output"
                            .to_string(),
                    );
                }
            }
        }

        Ok(JsonAction {
            action_type: func_call.name.clone(),
            args,
        })
    }

    fn convert_choice_item(choice_item: &ChoiceItem) -> Result<JsonChoice, String> {
        let condition = match &choice_item.condition {
            Some(Condition::Identifier(id)) => Some(JsonCondition {
                condition_type: id.clone(),
                args: Vec::new(),
            }),
            Some(Condition::FuncCall(func_call)) => Some(JsonCondition {
                condition_type: func_call.name.clone(),
                args: func_call
                    .args
                    .iter()
                    .map(|arg| match arg {
                        Arg::String(s) => s.clone(),
                        Arg::Number(n) => n.to_string(),
                        Arg::Boolean(b) => b.to_string(),
                        Arg::Identifier(id) => id.clone(),
                        Arg::FuncCall(_) => "nested_call".to_string(), // Simplified
                    })
                    .collect(),
            }),
            None => None,
        };

        let (next, action, nested_choice) = match &choice_item.target {
            ChoiceDest::Identifier(name, _) => (Some(name.clone()), None, None),
            ChoiceDest::Return => (None, Some("return".to_string()), None),
            ChoiceDest::Break => (None, Some("break".to_string()), None),
            ChoiceDest::NestedChoices(nested_items) => {
                let mut nested_choices = Vec::new();
                for item in nested_items {
                    nested_choices.push(Self::convert_choice_item(item)?);
                }
                (None, None, Some(nested_choices))
            }
        };

        Ok(JsonChoice {
            text: choice_item.text.clone(),
            condition,
            next,
            action,
            choice: nested_choice,
        })
    }

    fn convert_function_decl(func_decl: &FunctionDecl) -> JsonFunction {
        let params = func_decl
            .params
            .iter()
            .map(|param| JsonParam {
                name: param.name.clone(),
                param_type: param.type_name.clone(),
            })
            .collect();

        JsonFunction {
            name: func_decl.name.clone(),
            params,
            return_type: func_decl.return_type.clone(),
        }
    }

    fn convert_var_decl(var_decl: &VarDecl) -> JsonVariable {
        JsonVariable {
            name: var_decl.name.clone(),
            var_type: var_decl.type_name.clone(),
            value: var_decl.value.as_ref().map(Self::convert_var_value),
        }
    }

    fn convert_const_decl(const_decl: &ConstDecl) -> JsonConstant {
        JsonConstant {
            name: const_decl.name.clone(),
            const_type: const_decl.type_name.clone(),
            value: Self::convert_var_value(&const_decl.value),
            public: const_decl.is_public,
        }
    }

    fn convert_enum_def(enum_def: &EnumDef) -> JsonEnum {
        JsonEnum {
            name: enum_def.name.clone(),
            variants: enum_def.variants.clone(),
        }
    }

    fn convert_var_value(value: &VarValue) -> serde_json::Value {
        match value {
            VarValue::String(s) => serde_json::Value::String(s.clone()),
            VarValue::Number(n) => serde_json::json!(n),
            VarValue::Boolean(b) => serde_json::Value::Bool(*b),
        }
    }

    fn convert_interpolated_string(
        interpolated: &InterpolatedString,
    ) -> Result<(String, Vec<JsonStringPart>), String> {
        let mut rendered_text = String::new();
        let mut parts = Vec::new();

        for part in &interpolated.parts {
            match part {
                StringPart::Text(text) => {
                    rendered_text.push_str(text);
                    parts.push(JsonStringPart {
                        part_type: "text".to_string(),
                        content: text.clone(),
                        function_name: None,
                        args: Vec::new(),
                        enum_type: None,
                        branches: None,
                    });
                }
                StringPart::Expression(func_call) => {
                    // For rendering, we'll use a placeholder
                    let placeholder = format!("{{{}}}", func_call.name);
                    rendered_text.push_str(&placeholder);

                    // Convert arguments to strings
                    let args: Vec<String> = func_call
                        .args
                        .iter()
                        .map(|arg| {
                            match arg {
                                Arg::String(s) => format!("\"{}\"", s),
                                Arg::Number(n) => n.to_string(),
                                Arg::Boolean(b) => b.to_string(),
                                Arg::Identifier(id) => id.clone(),
                                Arg::FuncCall(nested) => format!("{}()", nested.name), // Simplified
                            }
                        })
                        .collect();

                    parts.push(JsonStringPart {
                        part_type: "expression".to_string(),
                        content: placeholder.clone(),
                        function_name: Some(func_call.name.clone()),
                        args,
                        enum_type: None,
                        branches: None,
                    });
                }
                StringPart::Placeholder(name) => {
                    // Placeholder will be resolved by branch definitions
                    let placeholder = format!("{{{}}}", name);
                    rendered_text.push_str(&placeholder);
                    
                    parts.push(JsonStringPart {
                        part_type: "placeholder".to_string(),
                        content: placeholder,
                        function_name: None,
                        args: Vec::new(),
                        enum_type: None,
                        branches: None,
                    });
                }
            }
        }

        Ok((rendered_text, parts))
    }
}
