use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DocNode {
    pub title: String,
    pub path: String,
    pub children: HashMap<String, DocNode>,
    pub content: Option<String>,
    pub depth: usize,
    pub position: Option<usize>,
}

impl DocNode {
    pub fn new(title: &str, path: &str) -> Self {
        DocNode {
            title: title.to_string(),
            path: path.to_string(),
            children: HashMap::new(),
            content: None,
            depth: 0,
            position: None,
        }
    }
}
