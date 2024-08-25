use std::collections::HashMap;

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
