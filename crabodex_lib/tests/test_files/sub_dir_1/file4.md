---
path:
  - Domain
  - Subdomain one
---

Documentation for the subdomain one

```rust
pub fn parse_front_matter(file_content: &str) -> Option<(String, Vec<String>, Option<usize>)> {
    if !file_content.starts_with(DEFAULT_FRONT_MATTER_PREFIX) { return None; }
    let content_after_prefix: &str = &file_content[DEFAULT_FRONT_MATTER_PREFIX.len()..];
    let end_index: Option<usize> = content_after_prefix.find(DEFAULT_FRONT_MATTER_PREFIX);
    if end_index.is_none() { return None; }
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
    let position: Option<usize> = doc["position"].as_i64().map(|p| p as usize);

    Some((title, path, position))
}
```
