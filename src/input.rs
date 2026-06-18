use crate::stdin;
use std::collections::HashMap;

//Input
pub fn user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid Input");
    return input.trim().to_string();
}

//Stopping the scan or continue
pub fn confirm() -> bool {
    println!("Type Y if it the wrong Folder/location, Otherwise Type Anything: ");
    let user_option = user_input();
    let user_option = user_option.trim();
    if user_option == "Y" {
        return false;
    } else {
        return true;
    }
}

pub fn category_con() -> bool {
    println!("Type Y if done with category, Otherwise Type Anything: ");
    let user_option = user_input();
    let user_option = user_option.trim();
    if user_option == "Y" {
        return false;
    } else {
        return true;
    }
}

pub fn custom_category_input(
    custom: &mut HashMap<String, Vec<String>>,
    folder_category: &mut Vec<String>,
) {
    println!("Enter a category name: ");
    let category = user_input();
    folder_category.push(category.clone());

    println!("Enter extension for {} (comma separated)", category);
    let extension = user_input();
    let extension: Vec<String> = extension
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    custom.insert(category.trim().to_string(), extension);
}
