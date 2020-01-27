use colored::*;
use web3::types::Address;

pub fn figlet() {
    println!(
        "
   / \\   _ __ __ _  ___ _ __ | |_ 
  / _ \\ | '__/ _` |/ _ | '_ \\| __|
 / ___ \\| | | (_| |  __| | | | |_ 
/_/   \\_|_|  \\__, |\\___|_| |_|\\__| -[ CLI
             |___/     
   "
    );
}

pub fn header(message: &str) {
    println!("\n{} {}", "-[".bold(), message.bold().cyan());
}

pub fn header_with_state(message: &str, state: &str) {
    println!("\n{} {} [{}]", "-[".bold(), message.bold().cyan(), state);
}

pub fn info(message: String) {
    println!("{}", message);
}

pub fn address(address: Address) {
    println!("{}", format!("{:#?}", address));
}

pub fn list(entries: &Vec<String>) {
    for entry in entries.iter() {
        println!("{}", entry);
    }
}

pub fn error(message: String) {
    println!("{} {} {}", "-[".bold(), "error".bold().red(), message);
}

pub fn end() {
    println!("");
}
