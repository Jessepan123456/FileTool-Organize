use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::rename;
use std::path::{Path, PathBuf};

use crate::json::save;

#[derive(Serialize, Deserialize)]
pub struct MoveRecord {
    pub from: PathBuf,
    pub to: PathBuf,
}

pub fn default_file_type(types: &str) -> &str {
    match types {
        "png" | "jpg" | "jpeg" | "gif" => "Images",
        "mp4" | "mov" | "avi" | "mkv" | "wmv" | "flv" => "Videos",
        "mp3" | "wav" | "m4a" | "flac" => "Music",
        "pdf" | "docx" | "txt" | "rtf" | "ppt" | "pptx" | "csv" | "dpc" => "Documents",
        "rs" | "js" | "java" | "html" | "css" | "ts" | "py" | "htm" | "json" => "Code",
        "zip" | "rar" | "7z" | "tar" => "Archives",
        _ => "Unknown",
    }
}

pub fn default_files_grouping(path: PathBuf, groups: &mut HashMap<String, Vec<PathBuf>>) {
    if let Some(types) = path.extension().and_then(|e| e.to_str()) {
        groups
            .entry(default_file_type(types).to_string())
            .or_default()
            .push(path);
    }
}

pub fn custom_files_grouping(
    path: PathBuf,
    groups: &mut HashMap<String, Vec<PathBuf>>,
    custom: &HashMap<String, Vec<String>>,
) {
    if let Some(types) = path.extension().and_then(|e| e.to_str()) {
        for (category, extensions) in custom {
            if extensions.iter().any(|e| e == types) {
                //Finds what matches
                groups
                    .entry(category.clone())
                    .or_default()
                    .push(path.clone());
            }
        }
    }
}

//moving all files into it own group
pub fn move_files(file_groups: &HashMap<String, Vec<PathBuf>>, path: &Path) {
    let mut record_history: Vec<MoveRecord> = Vec::new();
    for (key, files) in file_groups {
        for file in files {
            let file_name = match file.file_name() {
                Some(name) => name,
                None => {
                    println!("Could not get file");
                    continue;
                }
            };
            let new_path = path.join(key).join(file_name);

            let record = MoveRecord {
                from: file.clone(),
                to: new_path.clone(),
            };

            record_history.push(record);

            if let Err(e) = rename(file, new_path) {
                println!("Failed to move file: {}", e)
            }
        }
    }
    save(record_history);
}
