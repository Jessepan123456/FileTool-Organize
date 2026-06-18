use std::collections::HashMap;
use std::fs::{File, read_dir, rename};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::Error;
use crate::files::MoveRecord;
use crate::input::{confirm, user_input};

//Summary of files moved
pub fn summary(file_groups: &HashMap<String, Vec<PathBuf>>, location: &Path) {
    println!("Summary");
    for (key, files) in file_groups {
        println!("{}: {}", key, files.len())
    }
    println!("Move To: {}", &location.display());
}

pub fn save(record_history: Vec<MoveRecord>) {
    let json_record = match serde_json::to_string(&record_history) {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to save path: {}", e);
        }
    };

    if let Err(e) = std::fs::write("rollback.json", json_record) {
        panic!("Failed to write: {}", e);
    }
}

pub fn load() {
    let file = match File::open("rollback.json") {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to open json: {}", e);
        }
    };
    let reader = BufReader::new(file);

    let rollback_data: Vec<MoveRecord> = match serde_json::from_reader(reader) {
        Ok(d) => d,
        Err(_) => Vec::new(),
    };

    for record in rollback_data.iter().rev() {
        if let Err(e) = rename(record.to.clone(), record.from.clone()) {
            println!("Failed to move file: {}", e)
        }
    }
    clear();
}

pub fn clear() {
    let _file = File::create("rollback.json");
}

pub fn search() -> Result<(), Error> {
    loop {
        println!("Enter a folder path you want to search: ");
        let user_scan_folder = user_input();
        let user_scan_folder = user_scan_folder.trim();

        //Folder Path Start
        let folder_path = Path::new(user_scan_folder);

        println!("Enter you search by extension or filename/contain(case sensitive): ");
        let user_search = user_input();

        //Opens the Folder and explore what in it
        match read_dir(folder_path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(types) = path.extension().and_then(|e| e.to_str()) {
                            if types == user_search {
                                println!("{}", path.display())
                            }
                        }
                        if let Some(name) = path.file_name().and_then(|e| e.to_str()) {
                            if name.contains(&user_search) {
                                println!("{}", path.display())
                            }
                        }
                    }
                }
            }
            Err(_) => {
                println!("Invalid Folder Path");
            }
        }
        if confirm() == false {
            continue;
        } else {
            return Ok(());
        }
    }
}
