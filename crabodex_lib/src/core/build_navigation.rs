use crate::DocNode;

pub fn build_navigation(root: &DocNode) -> String {
    let mut html = String::new();
    html.push_str("<nav><ul>");

    let mut children: Vec<(&String, &DocNode)> = root.children.iter().collect();
    children.sort_by(|a, b| {
        match (a.1.position, b.1.position) {
            (Some(pos_a), Some(pos_b)) => pos_a.cmp(&pos_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.1.title.cmp(&b.1.title),
        }
    });

    for (_, child) in children {
        let child_id = child.path.replace(' ', "-").to_lowercase();
        html.push_str(&format!("<li><a href=\"#{}\">{}</a>", child_id, child.title));

        if !child.children.is_empty() {
            html.push_str(&build_navigation(child));
        }

        html.push_str("</li>");
    }

    html.push_str("</ul></nav>");
    html
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::find_markdown_files;
    use super::*;

    #[test]
    fn test_build_navigation() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir.clone());
        let doc_structure: DocNode = crate::core::build_doc_structure::build_doc_structure(&markdown_files);
        let navigation: String = build_navigation(&doc_structure);

        assert_eq!(navigation, "<nav><ul><li><a href=\"#getting-started\">Getting Started</a><nav><ul><li><a href=\"#getting-started->-configuration\">Configuration</a></li></ul></nav></li><li><a href=\"#features\">Features</a><nav><ul><li><a href=\"#features->-feature-one\">Feature one</a></li></ul></nav></li><li><a href=\"#domain\">Domain</a><nav><ul><li><a href=\"#domain->-subdomain-one\">Subdomain one</a></li><li><a href=\"#domain->-subdomain-two\">Subdomain two</a></li></ul></nav></li></ul></nav>");
    }
}