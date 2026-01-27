#![allow(unused_imports)]

use dashmap::mapref::entry;
use walkdir::{DirEntry, WalkDir};
use std::{fs::{File, Metadata}, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EntryType {
    File { size: u64, extension: String },
    Dir  { child_count: usize },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    name: String,
    path: PathBuf,
    kind: EntryType,
}

impl FileMetadata {
    // Contrutor do FileMetadata
    pub fn from_entry(entry: &DirEntry) -> Self {
        let path = entry.path().to_path_buf();

        let name = entry.file_name().to_string_lossy().to_string();
        let ft = entry.file_type();
        
        let kind = if ft.is_dir() {
            EntryType::Dir { child_count: 0 }
        } else {
            let extension = path.extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();
            
            // Pega o tamanho dos metadados que o WalkDir já tem
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            
            EntryType::File { size, extension }
        };

        FileMetadata { name, path, kind }
    }

    pub fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()   
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    // Check if is a Directory
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryType::Dir { .. })
    }

    // Check if is a file
    pub fn is_file(&self) -> bool {
        matches!(self.kind, EntryType::File { .. })
    }   

    // Clean the path and converts into PathBuf
    pub fn clean_path(input: &str) -> PathBuf {
        let cleaned = input.trim().trim_matches('"');
        PathBuf::from(cleaned)
    }

    pub fn list_all_by_path(path: &str) -> Vec<FileMetadata> {
        let node_path = FileMetadata::clean_path(path);

        WalkDir::new(node_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| !FileMetadata::is_hidden(e))
            .filter_map(|res| res.ok())             
            .inspect(|entry|{
                let kind = if entry.file_type().is_dir() { "Directory" }
                    else { "File" };
                println!("Type: {:?}, name: {:?}", kind, entry.file_name());
            })
            .map(|entry| FileMetadata::from_entry(&entry))
            .collect()
    } 
}