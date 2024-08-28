use std::path::{Component, Path, PathBuf};

use walkdir::WalkDir;
use std::collections::HashSet;

/// Find all markdown files in a directory and its subdirectories.
/// 
/// # Arguments
/// * `dir` - The directory to search for markdown files.
/// * `ignore_folders` - The folders to ignore.
/// 
/// # Returns
/// * `Vec<PathBuf>` - The list of markdown files.
/// 
/// # Panics
/// * If the directory does not exist.
/// * If the directory is not readable.
/// * If the directory is not a directory.
/// * If the directory is not a valid.
/// 
pub fn find_markdown_files<P: AsRef<Path>>(
    dir: P,
    ignore_folders: Option<&[&str]>,
) -> Vec<PathBuf> {
    let dir: &Path = dir.as_ref();
    let mut markdown_files: Vec<PathBuf> = Vec::new();
    let ignore_set: HashSet<&str> = ignore_folders.unwrap_or(&[]).iter().copied().collect();

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                // Check if the current directory should be ignored
                if entry.file_type().is_dir() && should_ignore(path, dir, &ignore_set) {
                    continue;
                }

                if entry.file_type().is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if extension == "md" {
                            if let Ok(relative_path) = entry.path().strip_prefix(dir) {
                                markdown_files.push(relative_path.to_path_buf());
                            }
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    markdown_files
}

fn should_ignore(path: &Path, base_dir: &Path, ignore_set: &HashSet<&str>) -> bool {
    let relative_path: &Path = match path.strip_prefix(base_dir) {
        Ok(rel_path) => rel_path,
        Err(_) => return false,
    };
    let first_component: Component = match relative_path.components().next() {
        Some(component) => component,
        None => return false,
    };
    let dir_name: &str = match first_component.as_os_str().to_str() {
        Some(name) => name,
        None => return false,
    };
    ignore_set.contains(dir_name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_find_markdown_files() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir, None);

        assert_eq!(markdown_files.len(), 7);
        assert!(markdown_files.iter().any(|p| p.file_name().unwrap() == "file1.md"));
        assert!(markdown_files.iter().any(|p| p.file_name().unwrap() == "file5.md"));
        assert!(markdown_files.iter().any(|p| p.ends_with("sub_dir_1/file4.md")));
        assert!(markdown_files.iter().any(|p| p.ends_with("sub_dir_1/sub_sub_dir/file3.md")));
        assert!(!markdown_files.iter().any(|p| p.file_name().unwrap() == "file.txt"));
    }
}
