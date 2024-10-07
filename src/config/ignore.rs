use globset::{GlobBuilder, GlobMatcher};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct GitIgnore {
    pat: Vec<GlobMatcher>,
}

impl GitIgnore {
    pub fn new() -> Self {
        Self { pat: Vec::new() }
    }

    pub fn parse_gitignore<P>(&mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let content = fs::read_to_string(path).expect("Failed to read .gitignore");
        let pat = content
            .lines()
            .filter_map(|line| {
                if line.trim().is_empty() || line.starts_with('#') {
                    None
                } else {
                    Some(
                        GlobBuilder::new(line)
                            .build()
                            .expect("Invalid glob pattern")
                            .compile_matcher(),
                    )
                }
            })
            .collect();

        Self { pat }
    }

    // TODO:
    pub fn should_ignore(&self, entry: &DirEntry) -> bool {
        let entry_path = entry.path();
        let mut matched = false;
        for m in &self.pat {
            if m.is_match(&entry_path) {
                matched = true;
                dbg!(m.glob().regex());
            }
        }
        dbg!(&entry_path);
        matched
    }

    pub fn get_filtered_entries<P>(&self, path: P) -> Vec<DirEntry>
    where
        P: AsRef<Path>,
    {
        fs::read_dir(path)
            .expect("Failed to read directory")
            .filter_map(Result::ok)
            .filter(|entry| !self.should_ignore(entry))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_file(path: &Path) {
        let mut file = File::create(path).expect("Failed to create test file");
        writeln!(file, "test").expect("Failed to write to test file");
    }

    fn setup_test_directory() -> tempfile::TempDir {
        let dir = tempdir().expect("Failed to create temp directory");

        let gitignore_path = dir.path().join(".gitignore");
        let test_file_path = dir.path().join("test.txt");
        let sub_dir_path = dir.path().join("sub_dir");
        let sub_file_path = sub_dir_path.join("test2.txt");

        fs::create_dir(&sub_dir_path).expect("Failed to create sub directory");
        create_test_file(&gitignore_path);
        create_test_file(&test_file_path);
        create_test_file(&sub_file_path);

        let mut file = File::create(&gitignore_path).expect("Failed to create .gitignore");
        writeln!(file, "test.txt\nsub_dir/").expect("Failed to write to .gitignore");

        dir
    }

    #[test]
    fn test_parse_gitignore() {
        let dir = setup_test_directory();
        let mut gitignore = GitIgnore::new();
        gitignore = gitignore.parse_gitignore(dir.path().join(".gitignore"));

        assert_eq!(gitignore.pat.len(), 2);
    }

    // cargo test -- test_should_ignore --nocapture
    #[test]
    fn test_should_ignore() {
        let dir = setup_test_directory();
        let mut gitignore = GitIgnore::new();
        gitignore = gitignore.parse_gitignore(dir.path().join(".gitignore"));

        let test_file = fs::read_dir(dir.path())
            .unwrap()
            .find(|entry| entry.as_ref().unwrap().file_name() == "test.txt")
            .unwrap()
            .unwrap();

        let sub_dir = fs::read_dir(dir.path())
            .unwrap()
            .find(|entry| entry.as_ref().unwrap().file_name() == "sub_dir")
            .unwrap()
            .unwrap();

        dbg!(&test_file);
        // dbg!(&gitignore);
        dbg!(&sub_dir);

        dbg!(gitignore.should_ignore(&test_file));
        dbg!(gitignore.should_ignore(&sub_dir));

        assert!(gitignore.should_ignore(&test_file));
        assert!(gitignore.should_ignore(&sub_dir));
    }

    // cargo test -- test_get_filtered_entries --nocapture
    #[test]
    fn test_get_filtered_entries() {
        let dir = setup_test_directory();
        let mut gitignore = GitIgnore::new();
        gitignore = gitignore.parse_gitignore(dir.path().join(".gitignore"));

        let entries = gitignore.get_filtered_entries(dir.path());

        dbg!(&entries);

        assert_eq!(entries.len(), 3);
        assert!(entries
            .iter()
            .any(|entry| entry.file_name() == ".gitignore"));
        assert!(entries.iter().any(|entry| entry.file_name() == "sub_dir"));
    }
}
