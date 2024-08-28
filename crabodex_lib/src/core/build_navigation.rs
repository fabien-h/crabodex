use crate::DocNode;

/// Build the navigation for the documentation recursively.
/// 
/// # Arguments
/// * `node` - The current node in the document structure.
/// 
/// # Returns
/// The navigation for the documentation as an HTML string.
/// 
#[must_use]
pub fn build_navigation(node: &DocNode) -> String {
    let mut html: String = String::default();
    html.push_str("<ul>");

    let mut children: Vec<(&String, &DocNode)> = node.children.iter().collect();
    children.sort_by(|a, b| {
        match (a.1.position, b.1.position) {
            (Some(pos_a), Some(pos_b)) => pos_a.cmp(&pos_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.1.title.cmp(&b.1.title),
        }
    });

    for (_, child) in children {
        let child_id: String = child.path.replace(' ', "-").to_lowercase();
        html.push_str(&format!("<li><a href=\"#{}\">{}</a>", child_id, child.title));

        if !child.children.is_empty() {
            html.push_str(&build_navigation(child));
        }

        html.push_str("</li>");
    }

    html.push_str("</ul>");
    html
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::core::build_doc_structure::build_doc_structure;
    use crate::find_markdown_files;
    use super::*;

    #[test]
    fn test_build_navigation() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir.clone(), &["ignored_test_files".to_string()]);
        let doc_structure: DocNode = build_doc_structure(&markdown_files, test_dir.as_path());
        let navigation: String = build_navigation(&doc_structure);

        assert_eq!(navigation, "<ul><li><a href=\"#getting-started\">Getting Started</a><ul><li><a href=\"#getting-started->-configuration\">Configuration</a></li></ul></li><li><a href=\"#features\">Features</a><ul><li><a href=\"#features->-feature-one\">Feature one</a></li></ul></li><li><a href=\"#domain\">Domain</a><ul><li><a href=\"#domain->-subdomain-one\">Subdomain one</a></li><li><a href=\"#domain->-subdomain-two\">Subdomain two</a></li></ul></li></ul>");
    }
}
