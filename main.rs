use std::fs;  

use std::io::{self, Write}; 

fn main() {  

// Ask user for file name println!("Please enter your file name:");  

let mut filename = String::new();  

io::stdin().read_line(&mut filename).expect("Failed to read line");  

let filename = filename.trim(); // remove trailing newline 

// Ask user for command 
println!("Please enter your command (backup, restore, delete):"); 
let mut command = String::new(); 
io::stdin().read_line(&mut command).expect("Failed to read line"); 
let command = command.trim(); 
 
// Match command and call corresponding function 
if command == "backup" { 
    backup_file(filename); 
} else if command == "restore" { 
    restore_file(filename); 
} else if command == "delete" { 
    delete_file(filename); 
} else { 
    println!("Unknown command"); 
} 
  

} 

fn backup_file(filename: &str) {  

let backup_name = format!("{}.bak", filename);  

// Copy original file to backup file  

match fs::copy(filename, &backup_name) {  

Ok(_) => println!("Your Backup created: {}", backup_name),  

Err(e) => println!("Failed to backup file: {}", e),  

} 

} 

fn restore_file(filename: &str) {  

let backup_name = format!("{}.bak", filename);  

// Copy backup file to original file  

match fs::copy(&backup_name, filename) { 

Ok(_) => println!("File restored from: {}", backup_name),  

Err(e) => println!("Failed to restore file: {}", e),  

}  

} 

fn delete_file(filename: &str) { 

print!("Are you sure you want to delete {}? (yes/no): ", filename);  

io::stdout().flush().unwrap(); // Flush to show prompt immediately 

let mut confirm = String::new(); 
io::stdin().read_line(&mut confirm).expect("Failed to read line"); 
if confirm.trim().to_lowercase() == "yes" { 
    match fs::remove_file(filename) { 
        Ok(_) => println!("File deleted."), 
        Err(e) => println!("Failed to delete file: {}", e), 
    } 
} else { 
    println!("Delete canceled."); 
} 
  

} 