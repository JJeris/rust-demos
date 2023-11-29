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


// use std::fs;
// use std::io;

// fn main() {
//     let folder_path = "C:\\Program Files\\Blender Foundation"; // Replace this with your folder path

//     if let Err(err) = delete_folder(folder_path) {
//         println!("Error: {}", err);
//     } else {
//         println!("Folder deleted successfully!");
//     }
// }

// fn delete_folder(folder_path: &str) -> io::Result<()> {
//     fs::remove_dir_all(folder_path)
// }
// use std::process::Command;

// fn main() {
//     let output = Command::new("runas")
//         .args(&["/user:Administrator", "cmd", "del", "C:\\appps"])
//         .output()
//         .expect("Failed to execute command");
//     if output.status.success() {
//         println!("File deleted successfully");
//     } else {
//         println!("Failed to delete file");
//     }
// }


// use runas;

// fn shell() -> String {
//     #[cfg(windows)]
//     {
//         "cmd".to_string()
//     }
//     #[cfg(unix)]
//     {
//         std::env::var("SHELL").unwrap_or_else(|_| "bash".into())
//     }
// }

// fn main() {
//     println!("Starting a root shell:");
//     println!(
//         "Status: {}",
//         runas::Command::new(shell())
//             .status()
//             .expect("failed to execute")
//     );
// }

// use runas::Command;

// fn main() {
//     // Replace 'folder_to_delete' with the path of the folder you want to delete
//     let folder_path = "C:\\test_file";

//     match Command::new("cmd")
//         .args(&["/C", "rmdir", "/s", "/q", &folder_path])
//         .status()
//     {
//         Ok(status) => {
//             if status.success() {
//                 println!("Folder deleted successfully");
//             } else {
//                 eprintln!("Failed to delete folder");
//             }
//         }
//         Err(err) => {
//             eprintln!("Error executing command: {:?}", err);
//         }
//     }
// }
//? WOOOOOORKS
// use runas::Command;

// fn main() {
//     // Replace 'folder_to_delete' with the path of the folder you want to delete
//     let folder_path = "C:\\Program Files\\Blender Foundation\\Blender 3.3";

//     match Command::new("cmd")
//         .args(&["/C", "rmdir", "/s", "/q", &folder_path])
//         .status()
//     {
//         Ok(status) => {
//             if status.success() {
//                 println!("Folder deleted successfully");
//             } else {
//                 eprintln!("Failed to delete folder");
//             }
//         }
//         Err(err) => {
//             eprintln!("Error executing command: {:?}", err);
//         }
//     }
// }


// use runas::Command;

// fn main() {
//     // Replace 'folder_to_delete' with the path of the folder you want to delete
//     let folder_path = "C:\\Program Files\\Blender Foundation\\Blender 3.3";

//     match Command::new("powershell")
//         .args(&[
//             "-Command",
//             &format!(
//                 r#"Start-Process powershell -WindowStyle Hidden -Verb RunAs -ArgumentList '-Command "Remove-Item -Recurse -Force \"{}\""'"#,
//                 folder_path
//             ),
//         ])
//         .status()
//     {
//         Ok(status) => {
//             if status.success() {
//                 println!("Folder deletion initiated");
//             } else {
//                 eprintln!("Failed to initiate folder deletion");
//             }
//         }
//         Err(err) => {
//             eprintln!("Error executing command: {:?}", err);
//         }
//     }
// }

// use std::process::Command;
// use std::io;

// fn delete_folder_with_admin_privileges(folder_path: &str) -> io::Result<()> {
//     // Command to execute PowerShell silently to remove the folder
//     let cmd = format!(
//         r#"Start-Process powershell -Verb runAs -WindowStyle Hidden -ArgumentList '-Command Remove-Item -Path "{}" -Force -Recurse'"#,
//         folder_path
//     );

//     // Execute the command with elevated privileges
//     runas::Command::new("powershell")
//         .args(&["-Command", &cmd])
//         .status()
//         .map(|_| ())
// }

// fn main() {
//     let folder_path = "C:\\Program Files\\Blender Foundation\\Blender 3.3"; // Replace with the folder path you want to delete

//     if let Err(err) = delete_folder_with_admin_privileges(folder_path) {
//         eprintln!("Failed to delete folder: {}", err);
//     } else {
//         println!("Folder deleted successfully!");
//     }
// }



// fn main() {
//     // Replace 'folder_to_delete' with the path of the folder you want to delete
//     let folder_path = "C:\\Program Files\\Blender Foundation\\Blender 3.3";

//     match Command::new("powershell")
//         .args(&[
//             "-Command",
//             &format!(
//                 r#"Start-Process powershell -WindowStyle Hidden -Verb RunAs -ArgumentList '-Command "Start-Process powershell -WindowStyle Hidden -ArgumentList \"-Command Remove-Item -Recurse -Force \\\"{}\\\"\" -Verb RunAs"' "#,
//                 folder_path
//             ),
//         ])
//         .status()
//     {
//         Ok(status) => {
//             if status.success() {
//                 println!("Folder deletion initiated");
//             } else {
//                 eprintln!("Failed to initiate folder deletion");
//             }
//         }
//         Err(err) => {
//             eprintln!("Error executing command: {:?}", err);
//         }
//     }
// }

use runas::Command;

// fn main() {
//     // Replace 'folder_to_delete' with the path of the folder you want to delete
//     let folder_path = "C:\\Program Files\\Blender Foundation\\Blender 3.3";

//     match Command::new("cmd")
//         .args(&["/C", "rmdir", "/s", "/q", &folder_path])
//         .status()
//     {
//         Ok(status) => {
//             if status.success() {
//                 println!("Folder deleted successfully");
//             } else {
//                 eprintln!("Failed to delete folder");
//             }
//         }
//         Err(err) => {
//             eprintln!("Error executing command: {:?}", err);
//         }
//     }
// }

// use std::process::Command as StdCommand;
// use std::time::Duration;
// use std::io::{self, ErrorKind};

fn main() {
    // Replace 'folder_to_delete' with the path of the folder you want to delete
    let folder_path = "C:\\Program Files\\Blender Foundation\\Blender 3.3";

    if let Err(err) = Command::new("cmd")
        .args(&["/C", "rmdir", "/s", "/q", &folder_path])
        .status()
    {
        eprintln!("Error executing command: {:?}", err);
    } else {
        // Sleep for 5 seconds
        // std::thread::sleep(Duration::from_secs(5));
        println!("Done");
    }
}
