use std::path::{Path, PathBuf};
use walkdir::WalkDir;
pub fn find_markdown_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let dir: &Path = dir.as_ref();
    let mut markdown_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_find_markdown_files() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files: Vec<PathBuf> = find_markdown_files(test_dir);

        assert_eq!(markdown_files.len(), 7);
        assert!(markdown_files.iter().any(|p| p.file_name().unwrap() == "file1.md"));
        assert!(markdown_files.iter().any(|p| p.file_name().unwrap() == "file5.md"));
        assert!(markdown_files.iter().any(|p| p.ends_with("sub_dir_1/file4.md")));
        assert!(markdown_files.iter().any(|p| p.ends_with("sub_dir_1/sub_sub_dir/file3.md")));
        assert!(!markdown_files.iter().any(|p| p.file_name().unwrap() == "file.txt"));
    }
}
