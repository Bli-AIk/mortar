//! # file_handler.rs
//!
//! # file_handler.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Provides file system utilities for the compiler.
//!
//! 为编译器提供文件系统实用工具。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! Contains the `FileHandler` struct for safely reading source files.
//!
//! 包含用于安全读取源文件的 `FileHandler` 结构体。

use std::fs;
use std::io;

#[derive(Debug)]
pub enum FileError {
    IoError(io::Error),
    NotFound,
}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::IoError(error)
    }
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::IoError(err) => write!(f, "IO error: {}", err),
            FileError::NotFound => write!(f, "File not found"),
        }
    }
}

impl std::error::Error for FileError {}

/// Handle file operations
pub struct FileHandler;

impl FileHandler {
    /// Read and validate the source file
    pub fn read_source_file(path: &str) -> Result<String, FileError> {
        fs::read_to_string(path).map_err(FileError::from)
    }
}
