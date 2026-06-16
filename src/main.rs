use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir};
use std::hash::Hash;
use std::io::{Error, stdin};
use std::path::{Path, PathBuf};

use crate::input::category_con;
use crate::input::user_input;

mod files;
mod input;
mod test;

// 1. Use default categories
// Images
// Videos
// Documents
// Code
// Archives
// 2. Create custom categories
//Enter category name:
// > Artwork

// Enter extensions for Artwork (comma separated):
// > psd,kra,clip

// Add another category? (Y/N)
// > Y

// Enter category name:
// > Music

// Enter extensions for Music:
// > mp3,wav,flac

// Add another category? (Y/N)
// > N
// Choice:

//Idea 2
//Rollback
//Save the folder before moving into a JSON and when you start the program it load it up if the user wants to rollback
//Show where it going to rollback to but it only saves the previous action

//Idea 3
//Search
//Allows the user to search category in there folder on there pc based on extension in a specific folder location
fn main() -> Result<(), Error> {
    println!(
        "
    1. Default Folders\n
    2. Custom Folders\n
    3. Rollback\n
    4. Search
    "
    );

    let input = user_input();
    let folder_choice = input.trim();
    match folder_choice {
        "1" => {}
        "2" => {
            let mut custom: HashMap<String, Vec<String>> = HashMap::new();
            println!("Enter a category name: ");
            let category = user_input();

            println!("Enter extension for {} (comma separated)", category);
            let extension = user_input();

            custom.entry(category).or_default().push(extension);

            while category_con() == true {
                println!("Enter a category name: ");
                let category = user_input();

                println!("Enter extension for {}(comma separated)", category);
                let extension = user_input();

                custom.entry(category).or_default().push(extension);
            }
            println!("{:?}", custom);
        }
        _ => {
            println!("Wrong")
        }
    }
    // let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // loop {
    //     println!("Enter a folder path you want to scan: ");
    //     let user_scan_folder = input::user_input();
    //     let user_scan_folder = user_scan_folder.trim();

    //     //Folder Path Start
    //     let folder_path = Path::new(user_scan_folder);
    //     file_groups.clear();

    //     //Opens the Folder and explore what in it
    //     match read_dir(folder_path) {
    //         Ok(entries) => {
    //             for entry in entries {
    //                 let entry = entry?;
    //                 let path = entry.path();

    //                 if path.is_file() {
    //                     println!("Found File: {:?}", path);
    //                     files::files_grouping(path, &mut file_groups);
    //                 } else {
    //                     println!("Found Folder: {:?}", path)
    //                 }
    //             }
    //         }
    //         Err(_) => {
    //             println!("Invalid Folder Path");
    //             continue;
    //         }
    //     }

    //     if input::confirm() == false {
    //         continue;
    //     } else {
    //         break;
    //     }
    // }

    // //Allow user to create there own folder location or default
    // let mut user_folder_location = String::new();
    // create_folders(&mut user_folder_location)?;
    // let user_folder_location = user_folder_location.trim();
    // let folder_path = Path::new(&user_folder_location);

    // summary(&file_groups, folder_path);
    // if input::confirm() == false {
    //     return Ok(());
    // } else {
    //     files::move_files(&file_groups, folder_path);
    // }

    Ok(())
}

// --- Helper Method ---

//Summary of files moved
fn summary(file_groups: &HashMap<String, Vec<PathBuf>>, location: &Path) {
    println!("Summary");
    for (key, files) in file_groups {
        println!("{}: {}", key, files.len())
    }
    println!("Move To: {}", &location.display());
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
