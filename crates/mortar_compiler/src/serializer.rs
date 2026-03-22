mod converters;
mod node_content;

use crate::Language;
use crate::ast::{
    Arg, AssignValue, ConstDecl, EnumDef, FunctionDecl, InterpolatedString, Program, StringPart,
    TopLevel, VarDecl, VarValue,
};
use crate::serializer_types::*;
use chrono::Utc;
use std::path::Path;

fn get_text(key: &str, language: Language) -> &'static str {
    match (key, language) {
        ("generated", Language::English) => "Generated:",
        ("generated", Language::Chinese) => "生成文件:",
        _ => "",
    }
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
            version: env!("CARGO_PKG_VERSION").to_string(),
            generated_at: Utc::now(),
        };

        let mut variables = Vec::new();
        let mut constants = Vec::new();
        let mut enums = Vec::new();
        let mut nodes = Vec::new();
        let mut functions = Vec::new();
        let mut events = Vec::new();
        let mut timelines = Vec::new();

        // Build event definitions map for reference resolution
        let mut event_map = std::collections::HashMap::new();
        for top_level in &program.body {
            if let TopLevel::EventDef(event_def) = top_level {
                event_map.insert(event_def.name.clone(), event_def);
            }
        }

        for top_level in &program.body {
            match top_level {
                TopLevel::NodeDef(node_def) => {
                    nodes.push(Self::convert_node_def(node_def, &event_map)?);
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
                TopLevel::EventDef(event_def) => {
                    events.push(Self::convert_event_def(event_def));
                }
                TopLevel::TimelineDef(timeline_def) => {
                    timelines.push(Self::convert_timeline_def(timeline_def));
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
            events,
            timelines,
        })
    }

    fn convert_arg_to_string(arg: &Arg) -> String {
        match arg {
            Arg::String(s) => format!("\"{}\"", s),
            Arg::Number(n) => n.to_string(),
            Arg::Boolean(b) => b.to_string(),
            Arg::Identifier(id) => id.clone(),
            Arg::FuncCall(fc) => format!("{}(...)", fc.name),
        }
    }

    fn convert_assign_value_to_string(value: &AssignValue) -> String {
        match value {
            AssignValue::EnumMember(enum_name, member) => format!("{}.{}", enum_name, member),
            AssignValue::Identifier(id) => id.clone(),
            AssignValue::Number(n) => n.to_string(),
            AssignValue::Boolean(b) => b.to_string(),
            AssignValue::String(s) => s.clone(),
        }
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
            VarValue::EnumMember(enum_name, member) => {
                serde_json::Value::String(format!("{}.{}", enum_name, member))
            }
            VarValue::Branch(branch_value) => {
                let cases: Vec<_> = branch_value
                    .cases
                    .iter()
                    .map(Self::convert_branch_case)
                    .collect();

                serde_json::json!({
                    "enum_type": branch_value.enum_type,
                    "cases": cases
                })
            }
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
                        .map(Self::convert_arg_to_string)
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
