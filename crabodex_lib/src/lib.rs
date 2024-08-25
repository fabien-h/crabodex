use std::path::{Path, PathBuf};
use crate::core::build_doc_structure::build_doc_structure;
use crate::core::build_navigation::build_navigation;
use crate::core::build_page_body::build_page_body;
use crate::core::create_html_document::create_html_document;
use crate::core::doc_node::DocNode;
use crate::core::find_markdown_files::find_markdown_files;

mod core;

pub fn generate<P: AsRef<Path>>(
    root_directory: P,
    repo_name: &str,
    repo_description: &str,
    commit_hash: &str,
    repo_url: &str
) -> String {
    let markdown_files: Vec<PathBuf> = find_markdown_files(&root_directory);
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
            "https://github.com/test_repo"
        );

        assert!(result.contains("<title>Crabodex -- Test repository</title>"));
    }
}
