use crate::stdin;

//Input
pub fn user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid Input");
    return input;
}

//Stopping the scan or continue
pub fn confirm() -> bool {
    println!("Type N if it the wrong Folder/location, Otherwise Type Anything: ");
    let user_option = user_input();
    let user_option = user_option.trim();
    if user_option == "N" {
        return false;
    } else {
        return true;
    }
}