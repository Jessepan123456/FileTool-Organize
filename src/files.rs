use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs::rename;

//determining File Types
pub fn file_type(types: &str) -> &str {
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

//Grouping Files Types
pub fn files_grouping(path: PathBuf, groups: &mut HashMap<String, Vec<PathBuf>>) {
    if let Some(types) = path.extension().and_then(|e| e.to_str()) {
        groups
            .entry(file_type(types).to_string())
            .or_default()
            .push(path);
    }
}

//moving all files into it own group
pub fn move_files(file_groups: &HashMap<String, Vec<PathBuf>>, path: &Path) {
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

            if let Err(e) = rename(file, new_path) {
                println!("Failed to move file: {}", e)
            }
        }
    }
}