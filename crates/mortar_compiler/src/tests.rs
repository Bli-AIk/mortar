//! # tests.rs
//!
//! # tests.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Defines the test modules for the `mortar_compiler` crate.
//!
//! 定义 `mortar_compiler` crate 的测试模块。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! Registers all integration and unit test modules.
//!
//! 注册所有集成测试和单元测试模块。

// Test modules
mod branch_test;
mod control_flow_test;
mod deserializer_test;
mod diagnostics_test;
#[cfg(test)]
mod file_handler_tests;
mod parser_test;
mod performance_serialization_test;
mod performance_test;
mod serializer_test;
mod token_test;
mod variable_test;
// Placeholder for future tests
// mod interpolation_test;
// mod separator_test;
// mod type_checking_test;
