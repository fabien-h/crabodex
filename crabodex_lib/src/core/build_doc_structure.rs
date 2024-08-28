use std::fs;
use std::path::{Path, PathBuf};
use crate::core::constants::DEFAULT_FRONT_MATTER_PREFIX;
use crate::core::parse_front_matter::parse_front_matter;
use crate::DocNode;

/// Build the document structure recursively.
/// 
/// # Arguments
/// * `files` - The list of markdown files to process.
/// * `root_directory` - The root directory where the markdown files are stored.
/// 
/// # Returns
/// The document structure as a `DocNode`.
/// 
/// # Panics
/// If the front matter is invalid.
#[must_use]
pub fn build_doc_structure(files: &[PathBuf], root_directory: &Path) -> DocNode {
    let mut root: DocNode = DocNode::new("Documentation", "");

    for file in files {
        let full_path: PathBuf = root_directory.join(file);
        let content: String = match fs::read_to_string(&full_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let (title, path, position): (String, Vec<String>, Option<usize>) = match parse_front_matter(&content) {
            Some((title, path, position)) => (title, path, position),
            None => continue,
        };

        let mut current: &mut DocNode = &mut root;
        let mut current_path: String = String::default();

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
        current.position = position;
        current.file_path = Some(file.to_string_lossy().to_string());
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

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir.clone(), None);
        let doc_structure: DocNode = build_doc_structure(&markdown_files, test_dir.as_path());

        assert!(doc_structure.children.contains_key("Getting Started"));
        let getting_started: &DocNode = &doc_structure.children["Getting Started"];
        assert_eq!(getting_started.title, "Getting Started");
    }
}
