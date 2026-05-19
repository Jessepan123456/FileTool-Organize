use std::collections::HashMap;
use std::fs::{create_dir, create_dir_all, read_dir, rename};
use std::io::Error;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    //Folder Path Start
    let folder_path = Path::new("../../../downloads");
    let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // 1. Create target folder
    // let new_folder = folder_path.join("RustPages");
    // create_dir(&new_folder)?;

    //Opens the Folder and explore what in it
    for entry in read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        //Print it out with it path
        if path.is_file() {
            files(path, &mut file_groups);
        } else {
            //Do Something with the folders IDK What yet
            // println!("Found Folder: {:?}", path)
        }

        // // 2. Build new file path
        // //Takes the file name of path
        // let file_name = path.file_name().unwrap();
        // //Combine it with new folder that was made
        // let new_path = new_folder.join(file_name);

        // // 3. Move file
        // //Move it
        // rename(&path, &new_path)?;

        // if let Some(types) = path.extension().and_then(|e| e.to_str()) {
        //     let result = file_type(types);
        //     println!("{}", result);
        // }
    }

    if Path::new("../../../Videos").exists() {
        println!("Videos Folder exists")
    } else if Path::new("../../../Images").exists() {
        println!("Images Folder exists")
    } else if Path::new("../../../Music").exists() {
        println!("Music Folder exists")
    } else if Path::new("../../../Documents").exists() {
        println!("Images Folder exists")
    } else if Path::new("../../../Code").exists() {
        println!("Code Folder exists")
    } else if Path::new("../../../Archives").exists() {
        println!("Archives Folder exists")
    } else {
        println!("Some Folder are missing")
    }

    // println!("{:?}", file_groups);
    Ok(())
}

//Helper Method for determining File Types
fn file_type(types: &str) -> &str {
    match types {
        "png" | "jpg" | "jpeg" | "gif" => "Images",
        "mp4" | "mov" => "Videos",
        "mp3" | "wav" => "Audio",
        "pdf" | "docx" => "Documents",
        "rs" | "js" | "java" => "Code",
        "zip" | "rar" => "Archives",
        _ => "Other Unknown File",
    }
}

//Helper Method for Grouping Files Types
fn files(path: PathBuf, groups: &mut HashMap<String, Vec<PathBuf>>) {
    if let Some(types) = path.extension().and_then(|e| e.to_str()) {
        groups
            .entry(file_type(types).to_string())
            .or_default()
            .push(path);
    }
}

// 1. Read Folder
// 2. Scan files
// 3. Detect File Type
// 4. Group Files by Type
// 5. Create Folders or if it already exist
// 6. Move to Folder
