use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir, rename};
use std::io::{Error, stdin};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    loop {
        println!("Enter a folder path you want to scan: ");
        let user_scan_folder = user_input();
        let user_scan_folder = user_scan_folder.trim();

        //Folder Path Start
        let folder_path = Path::new(user_scan_folder);
        file_groups.clear();

        //Opens the Folder and explore what in it
        match read_dir(folder_path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_file() {
                        println!("Found File: {:?}", path);
                        files_grouping(path, &mut file_groups);
                    } else {
                        println!("Found Folder: {:?}", path)
                    }
                }
            }
            Err(_) => {
                println!("Invalid Folder Path");
                continue;
            }
        }

        if confirm() == false {
            continue;
        } else {
            break;
        }
    }

    let mut user_folder_location = String::new();
    create_folders(&mut user_folder_location)?;
    let user_folder_location = user_folder_location.trim();
    let folder_path = Path::new(&user_folder_location);

    summary(&file_groups, folder_path);
    if confirm() == false {
        return Ok(());
    } else {
        move_files(&file_groups, folder_path);
    }

    Ok(())
}

// --- Helper Method ---

//getting user input
fn user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid Input");
    return input;
}

//Summary of files moved
fn summary(file_groups: &HashMap<String, Vec<PathBuf>>, location: &Path) {
    println!("Summary");
    for (key, files) in file_groups {
        println!("{}: {}", key, files.len())
    }
    println!("Move To: {}", &location.display());
}

//Stopping the scan or continue
fn confirm() -> bool {
    println!("Type N if it the wrong Folder/location, Otherwise Type Anything: ");
    let user_option = user_input();
    let user_option = user_option.trim();
    if user_option == "N" {
        return false;
    } else {
        return true;
    }
}

//determining File Types
fn file_type(types: &str) -> &str {
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
fn files_grouping(path: PathBuf, groups: &mut HashMap<String, Vec<PathBuf>>) {
    if let Some(types) = path.extension().and_then(|e| e.to_str()) {
        groups
            .entry(file_type(types).to_string())
            .or_default()
            .push(path);
    }
}

//Creating all the needed Folders
fn create_folders(user_folder_location: &mut String) -> Result<(), Error> {
    println!(
        "Enter a folder path for these folders(Images, Videos, Music, Documents, Code, Archives, Unknown): "
    );
    stdin()
        .read_line(user_folder_location)
        .expect("Invalid Input");
    let user_folder_location = user_folder_location.trim();

    let folders = [
        "Images",
        "Videos",
        "Music",
        "Documents",
        "Code",
        "Archives",
        "Unknown",
    ];

    let folder_path = Path::new(&user_folder_location);

    for folder in folders {
        create_dir_all(folder_path.join(folder))?;
    }

    Ok(())
}

//moving all files into it own group
fn move_files(file_groups: &HashMap<String, Vec<PathBuf>>, path: &Path) {
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
