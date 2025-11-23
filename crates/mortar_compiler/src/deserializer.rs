use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents the complete deserialized Mortar output structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MortaredData {
    pub metadata: Metadata,
    #[serde(default)]
    pub variables: Vec<Variable>,
    #[serde(default)]
    pub constants: Vec<Constant>,
    #[serde(default)]
    pub enums: Vec<Enum>,
    pub nodes: Vec<Node>,
    pub functions: Vec<Function>,
    #[serde(default)]
    pub events: Vec<EventDef>,
    #[serde(default)]
    pub timelines: Vec<TimelineDef>,
}

/// Metadata information about the compiled Mortar file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub version: String,
    pub generated_at: DateTime<Utc>,
}

/// A dialogue node
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub name: String,
    pub texts: Vec<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<BranchDef>>,
    #[serde(default)]
    pub variables: Vec<Variable>,
    #[serde(default)]
    pub runs: Vec<RunStmt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choice: Option<Vec<Choice>>,
}

/// A run statement
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunStmt {
    pub event_name: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_override: Option<IndexOverride>,
    #[serde(default)]
    pub ignore_duration: bool,
    pub position: usize,
}

/// Index override for run statements
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexOverride {
    #[serde(rename = "type")]
    pub override_type: String,
    pub value: String,
}

/// A branch definition for asymmetric text interpolation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BranchDef {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,
    pub cases: Vec<BranchCase>,
}

/// A single case in a branch definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BranchCase {
    pub condition: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
}

/// A text block with optional events and conditions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolated_parts: Option<Vec<StringPart>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<IfCondition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_statements: Vec<Statement>,
}

/// A statement (e.g., assignment)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Statement {
    #[serde(rename = "type")]
    pub stmt_type: String, // "assignment"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub var_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A conditional expression for if-else statements
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IfCondition {
    #[serde(rename = "type")]
    pub cond_type: String, // "binary", "unary", "identifier", "literal"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<Box<IfCondition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<Box<IfCondition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<IfCondition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A part of an interpolated string
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringPart {
    #[serde(rename = "type")]
    pub part_type: String, // "text", "expression", or "placeholder"
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<BranchCase>>,
}

/// An event triggered at a specific index
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub index: f64,
    pub actions: Vec<Action>,
}

/// An action to be executed
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(default)]
    pub args: Vec<String>,
}

/// A choice option for branching dialogue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choice: Option<Vec<Choice>>,
}

/// A condition for choice availability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    #[serde(rename = "type")]
    pub condition_type: String,
    #[serde(default)]
    pub args: Vec<String>,
}

/// A function declaration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    #[serde(default)]
    pub params: Vec<Param>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "return")]
    pub return_type: Option<String>,
}

/// A function parameter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

/// A variable declaration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub var_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// A constant declaration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constant {
    pub name: String,
    #[serde(rename = "type")]
    pub const_type: String,
    pub value: serde_json::Value,
    pub public: bool,
}

/// An enum definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

/// An event definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventDef {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    pub action: Action,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

/// A timeline definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimelineDef {
    pub name: String,
    pub statements: Vec<TimelineStmt>,
}

/// A timeline statement
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimelineStmt {
    #[serde(rename = "type")]
    pub stmt_type: String, // "run" or "wait"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(default)]
    pub ignore_duration: bool,
}

/// Deserializer for loading .mortared JSON files
pub struct Deserializer;

impl Deserializer {
    /// Deserialize a .mortared JSON string into MortaredData
    ///
    /// # Example
    /// ```
    /// use mortar_compiler::deserializer::Deserializer;
    ///
    /// let json_str = r#"{"metadata":{"version":"0.4.0","generated_at":"2025-11-20T00:00:00Z"},"nodes":[],"functions":[]}"#;
    /// let data = Deserializer::from_json(json_str).unwrap();
    /// assert_eq!(data.metadata.version, "0.4.0");
    /// ```
    pub fn from_json(json_str: &str) -> Result<MortaredData, String> {
        serde_json::from_str(json_str).map_err(|e| format!("Deserialization error: {}", e))
    }

    /// Load and deserialize a .mortared file from disk
    ///
    /// # Example
    /// ```no_run
    /// use mortar_compiler::deserializer::Deserializer;
    ///
    /// let data = Deserializer::from_file("dialogue.mortared").unwrap();
    /// println!("Loaded {} nodes", data.nodes.len());
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<MortaredData, String> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
        Self::from_json(&content)
    }

    /// Load and deserialize from a byte slice (useful for embedded resources)
    ///
    /// # Example
    /// ```
    /// use mortar_compiler::deserializer::Deserializer;
    ///
    /// let bytes = br#"{"metadata":{"version":"0.4.0","generated_at":"2025-11-20T00:00:00Z"},"nodes":[],"functions":[]}"#;
    /// let data = Deserializer::from_bytes(bytes).unwrap();
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<MortaredData, String> {
        serde_json::from_slice(bytes).map_err(|e| format!("Deserialization error: {}", e))
    }
}

impl MortaredData {
    /// Get a node by name
    ///
    /// # Example
    /// ```no_run
    /// # use mortar_compiler::deserializer::Deserializer;
    /// let data = Deserializer::from_file("dialogue.mortared").unwrap();
    /// if let Some(node) = data.get_node("Start") {
    ///     println!("Found node: {}", node.name);
    /// }
    /// ```
    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get a variable by name
    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find(|v| v.name == name)
    }

    /// Get a constant by name
    pub fn get_constant(&self, name: &str) -> Option<&Constant> {
        self.constants.iter().find(|c| c.name == name)
    }

    /// Get an enum by name
    pub fn get_enum(&self, name: &str) -> Option<&Enum> {
        self.enums.iter().find(|e| e.name == name)
    }

    /// Get an event definition by name
    pub fn get_event(&self, name: &str) -> Option<&EventDef> {
        self.events.iter().find(|e| e.name == name)
    }

    /// Get a timeline by name
    pub fn get_timeline(&self, name: &str) -> Option<&TimelineDef> {
        self.timelines.iter().find(|t| t.name == name)
    }

    /// Get all node names
    pub fn node_names(&self) -> Vec<&str> {
        self.nodes.iter().map(|n| n.name.as_str()).collect()
    }
}
