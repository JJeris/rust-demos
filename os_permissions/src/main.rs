use std::fs;

fn main() {
    // Hardcode the absolute path to the directory you want to check
    // let path = "C:\\blenderbaseapps";
    let path = "D:\\SteamLibrary\\steamapps";

    // Attempt to get the metadata for the provided path
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    // Check if the path is a directory
    if metadata.is_dir() {
        // Check if the directory is writable
        if metadata.permissions().readonly() {
            println!("The directory at {} is read-only.", path);
        } else {
            println!("The directory at {} is writable.", path);
        }
    } else {
        println!("The path at {} is not a directory.", path);
    }
}