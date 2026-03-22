//! # lib.rs
//!
//! # lib.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This is the library entry point for the Mortar language server. It re-exports the analysis,
//! backend, and file-tracking modules and exposes the `Backend` type that both the binary entry
//! point and tests use.
//!
//! 这是 Mortar 语言服务器的库入口。它重新导出分析、后端和文件跟踪模块，并公开 `Backend`
//! 类型，供二进制入口和测试共同使用。

pub mod analysis;
pub mod backend;
pub mod files;

pub use backend::Backend;
