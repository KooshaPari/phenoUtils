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
