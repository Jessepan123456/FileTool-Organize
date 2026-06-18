use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir};
use std::io::{Error, stdin};
use std::path::PathBuf;

use crate::Path;
use crate::files::{custom_files_grouping, default_files_grouping, move_files};
use crate::input::{user_input, confirm};
use crate::summary;

pub fn scan_folder(
    file_groups: &mut HashMap<String, Vec<PathBuf>>,
    custom: &mut HashMap<String, Vec<String>>,
    default: bool,
) -> Result<(), Error> {
    loop {
        println!("Enter a folder path you want to scan: ");
        let user_scan_folder = user_input();
        let user_scan_folder = user_scan_folder.trim();

        //Folder Path Start
        let folder_path = Path::new(user_scan_folder);
        file_groups.clear();

        //Opens the Folder and explore what in it
        read_folder(folder_path, default, file_groups, custom)?;

        if confirm() == false {
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

fn read_folder(
    folder_path: &Path,
    default: bool,
    file_groups: &mut HashMap<String, Vec<PathBuf>>,
    custom: &mut HashMap<String, Vec<String>>,
) -> Result<(), Error> {
    match read_dir(folder_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    println!("Found File: {:?}", path);
                    if default == true {
                        default_files_grouping(path, file_groups);
                    } else {
                        custom_files_grouping(path, file_groups, &custom);
                    }
                } else {
                    println!("Found Folder: {:?}", path)
                }
            }
        }
        Err(_) => {
            println!("Invalid Folder Path");
        }
    }
    Ok(())
}

pub fn create_folder(
    file_groups: &mut HashMap<String, Vec<PathBuf>>,
    folder_category: &mut Vec<String>,
    default: bool,
) -> Result<(), Error> {
    let mut user_folder_location = String::new();
    if default == true {
        default_create_folders(&mut user_folder_location)?;
    } else {
        custom_create_folders(&mut user_folder_location, folder_category)?;
    }
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

//Creating all the needed Folders
pub fn default_create_folders(user_folder_location: &mut String) -> Result<(), Error> {
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

//Creating all the needed Folders
pub fn custom_create_folders(
    user_folder_location: &mut String,
    folders: &mut Vec<String>,
) -> Result<(), Error> {
    println!("Enter a folder path for these folders: ");
    stdin()
        .read_line(user_folder_location)
        .expect("Invalid Input");
    let user_folder_location = user_folder_location.trim();

    let folder_path = Path::new(&user_folder_location);

    for folder in folders {
        create_dir_all(folder_path.join(folder))?;
    }

    Ok(())
}
