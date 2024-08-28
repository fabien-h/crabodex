
use yaml_rust::{Yaml, YamlLoader};
use crate::core::constants::DEFAULT_FRONT_MATTER_PREFIX;

/// Parses the front matter of a markdown file.
/// 
/// The front matter must be in YAML format.
/// The front matter must start with "---" and end with "---".
/// The front matter must contain a "path" field.
/// The "path" field must be a list of strings.
/// The last string in the "path" field is the title of the document.
/// The front matter may contain a "position" field.
/// The "position" field must be an integer.
/// 
/// # Arguments
/// * `file_content` - The content of the markdown file.
/// 
/// # Returns
/// * `Some((title, path, position))` - The title, path and position of the document.
/// * `None` - If the front matter is not found or is invalid.
/// 
/// # Panics
/// If the front matter is invalid.
/// If the "path" field is not found.
/// If the "path" field is not a list of strings.
/// If the "position" field is not an integer.
/// If the "position" field is not a valid integer.
/// If the "position" field is not a valid usize.
/// If the "position" field is not a valid index for the "path" field.
/// 
#[must_use]
pub fn parse_front_matter(file_content: &str) -> Option<(String, Vec<String>, Option<usize>)> {
    if !file_content.starts_with(DEFAULT_FRONT_MATTER_PREFIX) { return None; }
    let content_after_prefix: &str = &file_content[DEFAULT_FRONT_MATTER_PREFIX.len()..];
    let end_index: Option<usize> = content_after_prefix.find(DEFAULT_FRONT_MATTER_PREFIX);
    end_index?;
    let end_index: usize = end_index.unwrap();
    let front_matter: &str = &content_after_prefix[..end_index];

    let yaml: Vec<Yaml> = YamlLoader::load_from_str(front_matter).ok()?;
    if yaml.is_empty() { return None; }
    let doc: &Yaml = &yaml[0];

    let path: Vec<String> = match doc["path"].as_vec() {
        Some(path_yaml) => {
            let mut path: Vec<String> = Vec::new();
            for item in path_yaml {
                match item.as_str() {
                    Some(str_item) => path.push(str_item.to_string()),
                    None => return None,
                }
            }
            path
        },
        None => return None,
    };

    let title: String = path.last()?.clone();
    let position: Option<usize> = doc["position"].as_i64().and_then(|p| p.try_into().ok());

    Some((title, path, position))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_front_matter() {
        let file_content: &str = "---
path:
  - Section 1
  - Section 2
---

This is the content.";

        let result = parse_front_matter(file_content);
        assert!(result.is_some());
        let (title, path, position) = result.unwrap();
        assert_eq!(title, "Section 2");
        assert_eq!(path, vec!["Section 1", "Section 2"]);
        assert_eq!(position, None);
    }

    #[test]
    fn test_parse_front_matter_root_path() {
        let file_content: &str = "---
position: 1
path:
  - Test document
---

This is the content.";

        let result = parse_front_matter(file_content);
        assert!(result.is_some());
        let (title, path, position) = result.unwrap();
        assert_eq!(title, "Test document");
        assert_eq!(path, vec!["Test document"]);
        assert_eq!(position, Some(1));
    }
}
