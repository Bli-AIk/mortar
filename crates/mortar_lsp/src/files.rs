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
