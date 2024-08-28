use std::path::{Path, PathBuf};
use crate::core::build_doc_structure::build_doc_structure;
use crate::core::build_navigation::build_navigation;
use crate::core::build_page_body::build_page_body;
use crate::core::create_html_document::create_html_document;
use crate::core::doc_node::DocNode;
use crate::core::find_markdown_files::find_markdown_files;

pub mod core;

/// Generates a static documentation website from a directory containing markdown files.
/// 
/// 1. Finds all markdown files in the directory.
/// 2. Builds a tree structure of the documentation.
/// 3. Generates the navigation bar.
/// 4. Generates the page body.
/// 5. Creates the HTML document.
/// 6. Returns the HTML document as a string.
/// 7. Ignores folders specified in the `ignore_folders` argument.
/// 8. The `ignore_folders` argument is optional. If it is not provided, no folders will be ignored.
/// 
/// # Arguments
/// * `root_directory` - The directory containing the markdown files.
/// * `repo_name` - The name of the repository.
/// * `repo_description` - The description of the repository.
/// * `commit_hash` - The commit hash of the repository.
/// * `repo_url` - The URL of the repository.
/// * `ignore_folders` - The folders to ignore.
/// 
/// # Returns
/// * `String` - The HTML document as a string.
/// 
/// # Example
/// ```rust
/// use crabodex_lib::generate;
/// use std::path::PathBuf;
/// let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///   .join("tests")
///   .join("test_files");
/// let result: String = generate(
///    test_dir,
///    "Test repository",
///    "This repository helps us testing crabodex generation.",
///    "ebb34e7",
///    "https://github.com/crabodex/crabodex",
/// None);
/// ```
/// 
pub fn generate<P: AsRef<Path>>(
    root_directory: P,
    repo_name: &str,
    repo_description: &str,
    commit_hash: &str,
    repo_url: &str,
    ignore_folders: Option<&[&str]>,
) -> String {
    let markdown_files: Vec<PathBuf> = find_markdown_files(&root_directory, ignore_folders);
    let doc_structure: DocNode = build_doc_structure(&markdown_files, root_directory.as_ref());
    let navigation: String = build_navigation(&doc_structure);
    let page_body: String = build_page_body(&doc_structure, 0, repo_url);

    create_html_document(
        &navigation,
        &page_body,
        repo_name,
        repo_description,
        commit_hash,
        repo_url
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_find_markdown_files() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let result: String = generate(
            test_dir,
            "Test repository",
            "This repository helps us testing crabodex generation.",
            "ebb34e7",
            "https://github.com/test_repo",
            Some(&["ignored_test_files"]),
        );

        assert!(result.contains("<title>Test repository</title>"));
    }
}
