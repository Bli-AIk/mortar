//! # ast.rs
//!
//! # ast.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Defines the Abstract Syntax Tree (AST) for the Mortar language.
//!
//! 定义 Mortar 语言的抽象语法树 (AST)。
//!
//! These structures represent the parsed code in a hierarchical format, serving as the intermediate representation between parsing and code generation.
//!
//!这些结构以分层格式表示解析后的代码，充当解析和代码生成之间的中间表示。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! Contains the struct and enum definitions for all language constructs, including `Program`, `NodeDef`, `Event`, and `Statement`.
//!
//! 包含所有语言构造的结构体和枚举定义，包括 `Program`、`NodeDef`、`Event` 和 `Statement`。

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    NodeDef(NodeDef),
    FunctionDecl(FunctionDecl),
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    EnumDef(EnumDef),
    EventDef(EventDef),
    TimelineDef(TimelineDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub body: Vec<NodeStmt>,
    pub jump: Option<NodeJump>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStmt {
    Text(String),
    InterpolatedText(InterpolatedString),
    Choice(Vec<ChoiceItem>),
    Branch(BranchDef),
    IfElse(IfElseStmt),
    Run(RunStmt),
    WithEvents(WithEventsStmt),
    VarDecl(VarDecl),
    Assignment(Assignment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseStmt {
    pub condition: IfCondition,
    pub then_body: Vec<NodeStmt>,
    pub else_body: Option<Vec<NodeStmt>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfCondition {
    Binary(Box<BinaryCondition>),
    Unary(Box<UnaryCondition>),
    Identifier(String),
    EnumMember(String, String), // EnumName.member
    Literal(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryCondition {
    pub left: IfCondition,
    pub operator: ComparisonOp,
    pub right: IfCondition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryCondition {
    pub operator: UnaryOp,
    pub operand: IfCondition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    Equal,        // ==
    NotEqual,     // !=
    And,          // &&
    Or,           // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, // !
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedString {
    pub parts: Vec<StringPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Expression(FuncCall),
    Placeholder(String), // e.g., {place}, {object}
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchDef {
    pub name: String, // e.g., "place", "object"
    pub name_span: Option<(usize, usize)>,
    pub enum_type: Option<String>, // Some("ExampleEnum") or None for bool branches
    pub cases: Vec<BranchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchCase {
    pub condition: String, // e.g., "is_forest", "tree"
    pub text: String,
    pub events: Option<Vec<Event>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeJump {
    Identifier(String, Option<(usize, usize)>),
    Return,
    Break,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub index: f64,
    pub action: EventAction,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventAction {
    pub call: FuncCall,
    pub chains: Vec<FuncCall>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub index: Option<f64>,
    pub action: EventAction,
    pub duration: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub body: Vec<TimelineStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimelineStmt {
    Run(RunStmt),
    Wait(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunStmt {
    pub event_name: String,
    pub event_name_span: Option<(usize, usize)>,
    pub args: Vec<Arg>,
    pub index_override: Option<IndexOverride>,
    pub ignore_duration: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndexOverride {
    Value(f64),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithEventsStmt {
    pub events: Vec<WithEventItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WithEventItem {
    EventRef(String, Option<(usize, usize)>),
    EventRefWithOverride(String, Option<(usize, usize)>, IndexOverride), // Event name, span, index override
    InlineEvent(Event),
    EventList(Vec<WithEventItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceItem {
    pub text: String,
    pub condition: Option<Condition>,
    pub target: ChoiceDest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Identifier(String),
    FuncCall(FuncCall),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceDest {
    Identifier(String, Option<(usize, usize)>),
    Return,
    Break,
    NestedChoices(Vec<ChoiceItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCall {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    FuncCall(Box<FuncCall>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub type_name: String,
    pub value: Option<VarValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecl {
    pub is_public: bool,
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub type_name: String,
    pub value: VarValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Branch(BranchValue),
    EnumMember(String, String), // EnumName.member
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub var_name: String,
    pub var_name_span: Option<(usize, usize)>,
    pub value: AssignValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignValue {
    EnumMember(String, String), // EnumName.member
    Identifier(String),
    Number(f64),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchValue {
    pub enum_type: Option<String>, // Some("EnumType") for enum-based, None for bool-based
    pub cases: Vec<BranchCase>,
}