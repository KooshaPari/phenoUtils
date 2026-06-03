//! PhenoFS - Filesystem Utilities

use std::path::PathBuf;
use anyhow::Result;
use sha2::{Sha256, Digest};
use walkdir::WalkDir;

/// File entry with metadata
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub hash: Option<String>,
}

/// Recursively list directory contents
pub fn list_dir(path: impl AsRef<std::path::Path>) -> Result<Vec<FileEntry>> {
    let path = path.as_ref();
    
    let entries: Vec<FileEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| FileEntry {
            path: e.path().to_path_buf(),
            size: e.metadata().map(|m| m.len()).unwrap_or(0),
            is_dir: e.file_type().is_dir(),
            hash: None,
        })
        .collect();
    
    Ok(entries)
}

/// Compute SHA256 hash of file contents
pub fn compute_hash(path: impl AsRef<std::path::Path>) -> Result<String> {
    use std::fs;
    
    let contents = fs::read(path.as_ref())?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Atomic write with temp file and rename
pub fn atomic_write(path: impl AsRef<std::path::Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    use std::fs;
    
    let path = path.as_ref();
    let temp_path = path.with_extension("tmp");
    
    fs::write(&temp_path, contents.as_ref())?;
    fs::rename(&temp_path, path)?;
    
    Ok(())
}

/// Copy directory recursively
pub fn copy_dir(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> Result<u64> {
    use std::fs;
    
    let src = src.as_ref();
    let dst = dst.as_ref();
    let mut count = 0u64;
    
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let relative = entry.path().strip_prefix(src)?;
        let target = dst.join(relative);
        
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            fs::copy(entry.path(), &target)?;
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn atomic_write_then_read_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("data.bin");
        let payload = b"phenotype atomic payload";
        atomic_write(&p, payload).unwrap();
        assert_eq!(fs::read(&p).unwrap(), payload);
        assert!(!p.with_extension("tmp").exists(), "temp sidecar must not linger");
    }

    #[test]
    fn atomic_write_overwrites_existing() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("data.txt");
        atomic_write(&p, b"first").unwrap();
        atomic_write(&p, b"second").unwrap();
        assert_eq!(fs::read(&p).unwrap(), b"second");
    }

    #[test]
    fn compute_hash_is_known_and_stable() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("hash.txt");
        atomic_write(&p, b"abc").unwrap();
        // SHA-256("abc")
        assert_eq!(
            compute_hash(&p).unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn compute_hash_differs_for_different_contents() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a");
        let b = dir.path().join("b");
        atomic_write(&a, b"one").unwrap();
        atomic_write(&b, b"two").unwrap();
        assert_ne!(compute_hash(&a).unwrap(), compute_hash(&b).unwrap());
    }

    #[test]
    fn list_dir_finds_nested_entries() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        atomic_write(dir.path().join("sub/f.txt"), b"x").unwrap();
        let entries = list_dir(dir.path()).unwrap();
        assert!(entries.iter().any(|e| e.path.ends_with("f.txt") && !e.is_dir && e.size == 1));
        assert!(entries.iter().any(|e| e.path.ends_with("sub") && e.is_dir));
    }

    #[test]
    fn copy_dir_copies_files_and_returns_count() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        fs::create_dir(src.path().join("nested")).unwrap();
        atomic_write(src.path().join("a.txt"), b"alpha").unwrap();
        atomic_write(src.path().join("nested/b.txt"), b"beta").unwrap();
        let n = copy_dir(src.path(), dst.path()).unwrap();
        assert_eq!(n, 2, "two files copied");
        assert_eq!(fs::read(dst.path().join("a.txt")).unwrap(), b"alpha");
        assert_eq!(fs::read(dst.path().join("nested/b.txt")).unwrap(), b"beta");
    }
}
