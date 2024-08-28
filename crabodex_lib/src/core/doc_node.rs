use std::collections::HashMap;

/// A node in the documentation tree.
/// - `children`: The children of the node.
/// - `content`: The content of the node.
/// - `depth`: The depth of the node in the tree.
/// - `file_path`: The path of the file containing the node.
/// - `path`: The path of the node.
/// - `position`: The position of the node in the documentation structure.
/// - `title`: The title of the node.
/// 
#[derive(Debug, Clone)]
pub struct DocNode {
    pub children: HashMap<String, DocNode>,
    pub content: Option<String>,
    pub depth: usize,
    pub file_path: Option<String>,
    pub path: String,
    pub position: Option<usize>,
    pub title: String,
}

impl DocNode {
    /// Create a new `DocNode`.
    /// - `title`: The title of the node.
    /// - `path`: The path of the node.
    #[must_use]
    pub fn new(title: &str, path: &str) -> Self {
        DocNode {
            children: HashMap::new(),
            content: None,
            depth: 0,
            file_path: None,
            path: path.to_string(),
            position: None,
            title: title.to_string(),
        }
    }
}
