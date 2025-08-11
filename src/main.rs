use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    // Prompt for file name
    print!("Please enter your file name: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim();

    if filename.is_empty() {
        eprintln!("Filename cannot be empty.");
        return;
    }

    // Prompt for command
    print!("Please enter your command (backup, restore, delete): ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    let command = command.trim().to_lowercase();

    if !["backup", "restore", "delete"].contains(&command.as_str()) {
        eprintln!("Unknown command.");
        return;
    }

    // Dispatch command
    let result = match command.as_str() {
        "backup" => backup_file(filename),
        "restore" => restore_file(filename),
        "delete" => delete_file(filename),
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

fn backup_file(filename: &str) -> io::Result<()> {
    let backup_name = format!("{}.bak", filename);

    // Check if original file exists
    if !Path::new(filename).exists() {
        eprintln!("Original file '{}' does not exist.", filename);
        return Ok(());
    }

    fs::copy(filename, &backup_name)?;
    println!("Your backup created: {}", backup_name);
    log_action(&format!("Performed backup on {}\n", filename))?;
    Ok(())
}

fn restore_file(filename: &str) -> io::Result<()> {
    let backup_name = format!("{}.bak", filename);

    if !Path::new(&backup_name).exists() {
        eprintln!("Backup file '{}' does not exist.", backup_name);
        return Ok(());
    }

    fs::copy(&backup_name, filename)?;
    println!("File restored from: {}", backup_name);
    log_action(&format!("Performed restore on {}\n", filename))?;
    Ok(())
}

fn delete_file(filename: &str) -> io::Result<()> {
    if !Path::new(filename).exists() {
        eprintln!("File '{}' does not exist.", filename);
        return Ok(());
    }

    print!("Are you sure you want to delete {}? (yes/no): ", filename);
    io::stdout().flush()?;

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    if confirm.trim().eq_ignore_ascii_case("yes") {
        fs::remove_file(filename)?;
        println!("File deleted.");
        log_action(&format!("Performed delete on {}\n", filename))?;
    } else {
        println!("Delete canceled.");
    }
    Ok(())
}

fn log_action(action: &str) -> io::Result<()> {
    let mut log_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("logfile.txt")?;
    log_file.write_all(action.as_bytes())?;
    Ok(())
}
