//! # files.rs
//!
//! # files.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Implements the small bidirectional file-id map used by the LSP backend. It assigns
//! stable numeric ids to document URIs so other backend subsystems can refer to tracked files
//! without carrying duplicate lookup tables.
//!
//! 实现了 LSP backend 使用的小型双向文件 id 映射。它会为文档 URI 分配稳定的数字
//! 标识，好让其他后端子系统无需维护重复查找表也能引用已跟踪文件。

use std::collections::HashMap;

use tower_lsp_server::lsp_types::Uri;

pub type FileId = usize;

#[derive(Debug)]
pub struct Files {
    id_to_url: HashMap<FileId, Uri>,
    url_to_id: HashMap<Uri, FileId>,
    next_id: FileId,
}

impl Default for Files {
    fn default() -> Self {
        Self::new()
    }
}

impl Files {
    pub fn new() -> Self {
        Self {
            id_to_url: HashMap::new(),
            url_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    /// Insert new file or get existing file ID
    pub fn insert(&mut self, url: Uri) -> FileId {
        if let Some(&id) = self.url_to_id.get(&url) {
            return id;
        }

        let id = self.next_id;
        self.next_id += 1;

        self.id_to_url.insert(id, url.clone());
        self.url_to_id.insert(url, id);

        id
    }

    /// Find URL by ID
    pub fn get_url(&self, id: FileId) -> Option<&Uri> {
        self.id_to_url.get(&id)
    }

    /// Find ID by URL
    pub fn get_id(&self, url: &Uri) -> Option<FileId> {
        self.url_to_id.get(url).copied()
    }

    /// Get an iterator over all files
    pub fn iter(&self) -> impl Iterator<Item = (FileId, &Uri)> {
        self.id_to_url.iter().map(|(&id, url)| (id, url))
    }
}
