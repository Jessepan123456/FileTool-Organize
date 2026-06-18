use std::collections::HashMap;
use std::io::{Error, stdin};
use std::path::{Path, PathBuf};

use crate::folder::{create_folder, scan_folder};
use crate::input::{category_con, custom_category_input, user_input};
use crate::json::{load, search};

mod files;
mod folder;
mod input;
mod json;
mod test;

//Idea 3
//Search
//Allows the user to search category in there folder on there pc based on extension in a specific folder location
fn main() -> Result<(), Error> {
    loop {
        println!(
            "
        1. Default Folders\n
        2. Custom Folders\n
        3. Rollback(previous save)\n
        4. Search
        "
        );

        let mut custom: HashMap<String, Vec<String>> = HashMap::new();
        let mut folder_category = Vec::new();
        let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        let input = user_input();
        let folder_choice = input.trim();
        match folder_choice {
            "1" => {
                scan_folder(&mut file_groups, &mut custom, true)?;
                create_folder(&mut file_groups, &mut folder_category, true)?;
                break;
            }
            "2" => {
                //Custom Category
                custom_category_input(&mut custom, &mut folder_category);
                while category_con() == true {
                    custom_category_input(&mut custom, &mut folder_category);
                }
                scan_folder(&mut file_groups, &mut custom, false)?;
                create_folder(&mut file_groups, &mut folder_category, false)?;
                break;
            }
            "3" => {
                load();
                println!("Finish Rollback");
                break;
            }
            "4" => {
                search()?;
                break;
            }
            _ => {
                println!("Invalid Input")
            }
        }
    }

    Ok(())
}

