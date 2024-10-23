use pulldown_cmark::{html::push_html, CowStr, Event, Options, Parser, Tag, TagEnd};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use crate::DocNode;

/// Builds the body of the HTML page from the document structure.
///
/// # Arguments
/// * `root` - The root of the document structure.
/// * `depth` - The depth of the current node in the document structure.
/// * `repo_url` - The URL of the repository where the markdown files are stored.
///
/// # Returns
/// The body of the HTML page.
///
/// # Panics
/// If the Front Matter is invalid.
///
#[must_use]
pub fn build_page_body(root: &DocNode, depth: usize, repo_url: &str) -> String {
    let mut html: String = String::default();
    let mut options: Options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
    let theme = &theme_set.themes["base16-eighties.dark"];

    if depth > 0 {
        let header_level: usize = std::cmp::min(depth, 6);
        let id: String = root.path.replace(' ', "-").to_lowercase();

        if root.file_path.is_some() {
            let github_link: String =
                format!("{}/blob/main/{}", repo_url, root.clone().file_path.unwrap());
            html.push_str(&format!(
                "<h{0} id=\"{1}\"><span>{2} <a href=\"{3}\" title=\"View on Github\" class=\"gh-icon\"></a></span></h{0}>",
                header_level,
                id,
                root.title,
                github_link
            ));
        } else {
            html.push_str(&format!(
                "<h{0} id=\"{1}\"><span>{2}</span></h{0}>",
                header_level, id, root.title
            ));
        }

        if let Some(content) = &root.content {
            let mut content_html: String = String::with_capacity(&content.len() * 3 / 2);
            let parser: Parser = Parser::new_ext(content, options);

            // We'll build a new vector of events since we can only consume the parser once
            let mut new_p = Vec::new();
            // As we go along, we'll want to highlight code in bundles, not lines
            let mut to_highlight = String::new();
            // And track a little bit of state
            let mut in_code_block = false;

            for event in parser {
                match event {
                    Event::Start(Tag::CodeBlock(_)) => {
                        in_code_block = true;
                    }
                    Event::End(TagEnd::CodeBlock) => {
                        if in_code_block {
                            // Format the whole multi-line code block as HTML all at once
                            let html = highlighted_html_for_string(
                                &to_highlight,
                                &syntax_set,
                                syntax,
                                theme,
                            )
                            .unwrap();
                            // And put it into the vector
                            new_p.push(Event::Html(CowStr::Boxed(html.into())));
                            to_highlight = String::new();
                            in_code_block = false;
                        }
                    }
                    Event::Text(t) => {
                        if in_code_block {
                            // If we're in a code block, build up the string of text
                            to_highlight.push_str(&t);
                        } else {
                            new_p.push(Event::Text(t))
                        }
                    }
                    e => {
                        new_p.push(e);
                    }
                }
            }

            push_html(&mut content_html, new_p.into_iter());
            html.push_str(&format!(
                "<div class=\"depth-{depth}\">{content_html}</div>"
            ));
        }
    }

    let mut children: Vec<(&String, &DocNode)> = root.children.iter().collect();
    children.sort_by(|a, b| match (a.1.position, b.1.position) {
        (Some(pos_a), Some(pos_b)) => pos_a.cmp(&pos_b),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.1.title.cmp(&b.1.title),
    });

    for (_, child) in children {
        html.push_str(&build_page_body(child, depth + 1, repo_url));
    }

    html
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::build_doc_structure::build_doc_structure;
    use crate::find_markdown_files;

    use super::*;

    #[test]
    fn test_build_page_body() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir.clone(), &[]);
        let doc_structure: DocNode = build_doc_structure(&markdown_files, test_dir.as_path());
        let page_body: String =
            build_page_body(&doc_structure, 0, "https://github.com/example/repo");

        assert!(page_body.contains("<h1 id=\"getting-started\"><span>Getting Started <a href=\"https://github.com/example/repo/blob/main/file1.md\" title=\"View on Github\" class=\"gh-icon\"></a></span></h1>"));
        assert!(page_body.contains("<h2 id=\"getting-started->-configuration\"><span>Configuration <a href=\"https://github.com/example/repo/blob/main/sub_dir_2/file2.md\" title=\"View on Github\" class=\"gh-icon\"></a></span></h2>"));
    }
}
