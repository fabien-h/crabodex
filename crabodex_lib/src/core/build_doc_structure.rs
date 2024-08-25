use std::fs;
use std::path::PathBuf;
use crate::core::constants::DEFAULT_FRONT_MATTER_PREFIX;
use crate::core::parse_front_matter::parse_front_matter;
use crate::DocNode;

fn build_doc_structure(files: &[PathBuf]) -> DocNode {
    let mut root: DocNode = DocNode::new("Documentation", "");

    for file in files {
        let content: String = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let (title, path): (String, Vec<String>) = match parse_front_matter(&content) {
            Some((title, path)) => (title, path),
            None => continue,
        };

        let mut current: &mut DocNode = &mut root;
        let mut current_path: String = String::new();

        for (i, section) in path.iter().enumerate() {
            if i > 0 { current_path.push_str(" > ") }
            current_path.push_str(section);

            current = current.children.entry(section.clone()).or_insert_with(|| {
                DocNode::new(
                    if i == path.len() - 1 { &title } else { section },
                    &current_path
                )
            });
        }

        if let Some(content_start) = content.find(DEFAULT_FRONT_MATTER_PREFIX).and_then(|i| {
            content[i + DEFAULT_FRONT_MATTER_PREFIX.len()..].find(DEFAULT_FRONT_MATTER_PREFIX)
        }) {
            let content_start = content.find(DEFAULT_FRONT_MATTER_PREFIX).unwrap() + DEFAULT_FRONT_MATTER_PREFIX.len() + content_start + DEFAULT_FRONT_MATTER_PREFIX.len();
            current.content = Some(content[content_start..].trim().to_string());
        }

        current.title = title;
        current.path = current_path;
        current.depth = path.len();
    }

    root
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::find_markdown_files;

    #[test]
    fn test_process_markdown_files() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir.clone());
        let doc_structure: DocNode = build_doc_structure(&markdown_files);

        assert!(doc_structure.children.contains_key("Getting Started"));
        let getting_started: &DocNode = &doc_structure.children["Getting Started"];
        assert_eq!(getting_started.title, "Introduction");
        assert!(getting_started.children.contains_key("Setup"));
        let setup: &DocNode = &getting_started.children["Setup"];
        assert_eq!(setup.title, "Configuration");
    }
}