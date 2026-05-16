use std::fs::{create_dir, create_dir_all};
use std::fs::{read_dir, rename};
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    //Folder Path Start
    let folder_path = Path::new("../weather/src/pages");

    //Opens the Folder and explore what in it
    for entry in read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        //Print it out with it path
        if path.is_file() {
            println!("Found File: {:?}", path);
        } else {
            println!("Found Folder: {:?}", path)
        }

        if let Some(types) = path.extension().and_then(|e| e.to_str()) {
            let result = file_type(types);
            println!("{}", result);
        }
    }

    Ok(())
}

//File Type Helper Method
fn file_type(types: &str) -> &str {
    match types {
        "rs" => "Rust File",
        _ => "Other Unknown File",
    }
}

// 1. Read Folder
// 2. Scan files
// 3. Detect File Type
// 4. Group Files by Type
// 5. Create Folders or if it already exist
// 6. Move to Folder
