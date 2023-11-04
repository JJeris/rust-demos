

// use std::fs::File;
// use std::path::Path;
// use zip::read::ZipArchive;

// fn main() {
//     let zip_file_path = "C:\\Users\\J\\Desktop\\test_folder\\test.zip";
//     let extract_path = "C:\\Users\\J\\Desktop\\test_folder\\";

//     match unzip(zip_file_path, extract_path) {
//         Ok(_) => println!("Extraction completed successfully!"),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }

// fn unzip(zip_file_path: &str, extract_path: &str) -> zip::result::ZipResult<()> {
//     let file = File::open(zip_file_path)?;
//     let mut archive = ZipArchive::new(file)?;

//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i)?;
//         let file_path = file.sanitized_name();

//         let file_dest_path = Path::new(extract_path).join(file_path);

//         if let Some(parent) = file_dest_path.parent() {
//             std::fs::create_dir_all(parent)?;
//         }

//         if file.is_dir() {
//             std::fs::create_dir_all(&file_dest_path)?;
//         } else {
//             if !file_dest_path.exists() {
//                 let mut dest_file = File::create(&file_dest_path)?;
//                 std::io::copy(&mut file, &mut dest_file)?;
//             }
//         }
//     }

//     Ok(())
// }

use std::fs::File;
use std::path::Path;
use zip::read::ZipArchive;

fn main() {
    let zip_file_path = "C:\\Users\\J\\Desktop\\test_folder\\test.zip";
    let extract_path = "C:\\Users\\J\\Desktop\\test_folder\\";

    match unzip(zip_file_path, extract_path) {
        Ok(_) => println!("Extraction completed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn unzip(zip_file_path: &str, extract_path: &str) -> zip::result::ZipResult<()> {
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let total_files = archive.len();

    for i in 0..total_files {
        let mut file = archive.by_index(i)?;
        let file_path = file.mangled_name();

        let file_dest_path = Path::new(extract_path).join(file_path);

        if let Some(parent) = file_dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if file.is_dir() {
            std::fs::create_dir_all(&file_dest_path)?;
        } else {
            if !file_dest_path.exists() {
                let mut dest_file = File::create(&file_dest_path)?;
                std::io::copy(&mut file, &mut dest_file)?;
            }
        }

        let progress = ((i + 1) as f32 / total_files as f32 * 100.0) as u32;
        print!("\rExtracting... {}%", progress);
    }

    println!("\nExtraction completed!");
    Ok(())
}