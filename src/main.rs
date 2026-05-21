use std::collections::HashMap;
use std::fs::{create_dir, create_dir_all, read_dir, rename};
use std::io::{Error, stdin};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    loop {
        let mut user_folder = String::new();
        let mut user_option = String::new();

        //Safety Feature for when it fails to find the folder or scan it
        println!("Enter a folder path you want to scan: ");
        stdin().read_line(&mut user_folder).expect("Invalid");
        let user_folder = user_folder.trim();

        //Folder Path Start
        let folder_path = Path::new(user_folder);
        let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        //Opens the Folder and explore what in it
        match read_dir(folder_path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();

                    //Print it out with it path
                    if path.is_file() {
                        println!("Found File: {:?}", path);
                        files(path, &mut file_groups);
                    } else {
                        //Do Something with the folders IDK What yet
                        println!("Found Folder: {:?}", path)
                    }
                }
            }
            Err(_) => {
                println!("Invalid Folder Path");
                continue;
            }
        }

        println!("Type N if it the wrong Folder, Otherwise Type Anything: ");
        stdin().read_line(&mut user_option).expect("Invalid Input");
        let user_option = user_option.trim();
        if user_option == "N" {
            continue;
        } else {
            break;
        }
    }
    //Make it possible for those folder to be create on the desire location
    create_folders()?;

    // // 2. Build new file path
    // //Takes the file name of path
    // let file_name = path.file_name().unwrap();
    // //Combine it with new folder that was made
    // let new_path = new_folder.join(file_name);

    // // 3. Move file
    // //Move it
    // rename(&path, &new_path)?;

    // for (key, value) in &file_groups {
    //     println!("{}: {:?}", key, value);
    // }

    create_folders()?;
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

//Helper Method for Creating all the needed Folders
fn create_folders() -> Result<(), Error> {
    create_dir_all("../../../Images")?;
    create_dir_all("../../../Videos")?;
    create_dir_all("../../../Music")?;
    create_dir_all("../../../Documents")?;
    create_dir_all("../../../Code")?;
    create_dir_all("../../../Archives")?;
    create_dir_all("../../../Unknown")?;

    // 1. Create target folder
    // let new_folder = folder_path.join("RustPages");
    // create_dir(&new_folder)?;

    Ok(())
}

// 1. Read Folder
// 2. Scan files
// 3. Detect File Type
// 4. Group Files by Type
// 5. Create Folders or if it already exist
// 6. Move to Folder
