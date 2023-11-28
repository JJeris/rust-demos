// use std::fs;
// use std::fs::File;
// use std::io::Write;
// use std::path::{Path, PathBuf};
// use dirs;

// fn main() {
//     let path: String = "C:\\Program Files\\Blender Foundation".to_string();
//     println!("{path}");
//     let metadata = fs::metadata(path).unwrap();
    
//     // Check if the path is a directory
//     if metadata.is_dir() {
//         // Check if the directory is writable
//         println!("{:#?}", metadata.permissions().readonly());

//     } else {
//         // The path is not a directory
//         println!("The provided path is not a directory.");
//     }
// }

// Sort of works.
// use std::fs;

// fn main() -> std::io::Result<()> {
//     let loc = "C:\\Program Files\\Blender Foundation";

//     // Check if able to write inside directory
//     let md = fs::metadata(loc)?;
//     let permissions = md.permissions();
//     let readonly = permissions.readonly();
//     if readonly {
//         println!("No write permission");
//     } else {
//         println!("Write permission");
//     }
//     Ok(())
// }


use std::fs;
use std::io;

fn main() {
    let folder_path = "C:\\Program Files\\Blender Foundation"; // Replace this with your folder path

    if let Err(err) = delete_folder(folder_path) {
        println!("Error: {}", err);
    } else {
        println!("Folder deleted successfully!");
    }
}

fn delete_folder(folder_path: &str) -> io::Result<()> {
    fs::remove_dir_all(folder_path)
}
