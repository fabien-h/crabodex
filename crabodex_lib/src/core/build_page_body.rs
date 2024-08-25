use pulldown_cmark::{Parser, html::push_html};
use crate::DocNode;

pub fn build_page_body(root: &DocNode, depth: usize) -> String {
    let mut html: String = String::new();

    if depth > 0 {
        let header_level = std::cmp::min(depth, 6);
        let id = root.path.replace(' ', "-").to_lowercase();
        html.push_str(&format!("<h{0} id=\"{1}\">{2}</h{0}>", header_level, id, root.title));

        // Compile markdown content
        if let Some(content) = &root.content {
            let mut content_html = String::new();
            let parser = Parser::new(content);
            push_html(&mut content_html, parser);
            html.push_str(&content_html);
        }
    }

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
        html.push_str(&build_page_body(child, depth + 1));
    }

    html
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::find_markdown_files;
    use crate::core::build_doc_structure::build_doc_structure;
    use super::*;

    #[test]
    fn test_build_page_body() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir.clone());
        let doc_structure: DocNode = build_doc_structure(&markdown_files);
        let page_body: String = build_page_body(&doc_structure, 0);

        assert!(page_body.contains("<h1 id=\"getting-started\">Getting Started</h1>"));
        assert!(page_body.contains("<h1 id=\"features\">Features</h1>"));
        assert!(page_body.contains("<h1 id=\"domain\">Domain</h1>"));

        assert!(page_body.contains("<h2 id=\"getting-started->-configuration\">Configuration</h2>"));
        assert!(page_body.contains("<h2 id=\"features->-feature-one\">Feature one</h2>"));
        assert!(page_body.contains("<h2 id=\"domain->-subdomain-one\">Subdomain one</h2>"));
        assert!(page_body.contains("<h2 id=\"domain->-subdomain-two\">Subdomain two</h2>"));

        let getting_started_index = page_body.find("Getting Started").unwrap();
        let features_index = page_body.find("Features").unwrap();
        let domain_index = page_body.find("Domain").unwrap();
        assert!(getting_started_index < features_index);
        assert!(features_index < domain_index);
    }
}