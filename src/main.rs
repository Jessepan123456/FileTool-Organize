use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir};
use std::io::{Error, stdin};
use std::path::{Path, PathBuf};

mod files;
mod input;
mod test;

fn main() -> Result<(), Error> {
    let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    loop {
        println!("Enter a folder path you want to scan: ");
        let user_scan_folder = input::user_input();
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
                        files::files_grouping(path, &mut file_groups);
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

        if input::confirm() == false {
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
    if input::confirm() == false {
        return Ok(());
    } else {
        files::move_files(&file_groups, folder_path);
    }

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
