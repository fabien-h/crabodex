use std::path::{ Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

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
    ignore_folders: &[String],
) -> Vec<PathBuf> {
    let dir: &Path = dir.as_ref();
    let mut markdown_files: Vec<PathBuf> = Vec::new();
    
    println!("ignore_folders: {:?}", ignore_folders);

    for entry in WalkDir::new(dir).follow_links(true) {


        let entry: DirEntry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };


        let is_ignored: bool = ignore_folders.iter().any(|needle|
            entry.path().to_str().map_or(false, |path| path.contains(needle))
        );
        if is_ignored { continue; }

        println!("entry: {:?}", entry);
        println!("entry path: {:?}", entry.path());

        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "md") {
            if let Ok(relative_path) = entry.path().strip_prefix(dir) {
                markdown_files.push(relative_path.to_path_buf());
            }
        }
    }

    println!("markdown_files: {:?}", markdown_files);

    markdown_files
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

        assert_eq!(markdown_files.len(), 8);
        assert!(markdown_files.iter().any(|p| p.file_name().unwrap() == "file1.md"));
        assert!(markdown_files.iter().any(|p| p.file_name().unwrap() == "file5.md"));
        assert!(markdown_files.iter().any(|p| p.ends_with("sub_dir_1/file4.md")));
        assert!(markdown_files.iter().any(|p| p.ends_with("sub_dir_1/sub_sub_dir/file3.md")));
        assert!(!markdown_files.iter().any(|p| p.file_name().unwrap() == "file.txt"));
    }

    #[test]
    fn test_ignore_folders() {
        let test_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_files");

        let markdown_files_without_ignore: Vec<PathBuf> = find_markdown_files(&test_dir, None);
        assert!(markdown_files_without_ignore.iter().any(|p| p.ends_with("ignored_test_files/file_8.md")));

        let ignore_folders: &[&str; 1] = &["ignored_test_files"];
        let markdown_files_with_ignore: Vec<PathBuf> = find_markdown_files(&test_dir, Some(ignore_folders));
        assert!(!markdown_files_with_ignore.iter().any(|p| p.ends_with("ignored_test_files/file_8.md")));
        assert!(markdown_files_with_ignore.iter().any(|p| p.ends_with("file1.md")));
    }
}
