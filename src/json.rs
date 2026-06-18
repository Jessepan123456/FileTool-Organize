use std::fs::{File, rename};
use std::io::BufReader;

use crate::files::MoveRecord;

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
