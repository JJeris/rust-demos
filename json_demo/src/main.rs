// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct Paragraph {
//     name: String
// }

// use regex::Regex;



// fn main() {
//     let input = "blender-3.6.4.md5";

//     // Create a regex pattern to capture the numeric part
//     let re = Regex::new(r#"\d+\.\d+"#).unwrap();

//     // Search for the numeric part in the input
//     if let Some(capture) = re.find(input) {
//         let matched = capture.as_str();
//         println!("Matched: {}", matched);
//     } else {
//         println!("No match found.");
//     }
// }

// use regex::Regex;

// fn extract_version(input: &str) -> Option<String> {
//     let re = Regex::new(r#"\d+\.\d+[a-z]*\d*(?:\.\d+)*"#).unwrap();

//     if let Some(capture) = re.find(input) {
//         let matched = capture.as_str();
//         Some(matched.to_string())
//     } else {
//         None
//     }
// }

// fn main() {
//     let strings = vec![
//         "blender-2.79-linux-glibc219-i686.tar.bz2",
//         "blender-2.79b-linux-glibc219-i686.tar.bz2",
//         "blender-3.3.7.md5",
//         "blender-2.80rc1-linux-glibc224-i686.tar.bz2",
//         "blender-2.83.3.0-windows64.msix",
//     ];

//     for s in strings {
//         if let Some(version) = extract_version(s) {
//             println!("Extracted: {}", version);
//         } else {
//             println!("No match found.");
//         }
//     }
// }

// use regex::Regex;

// fn extract_parts(input: &str) -> (String, String, String) {
//     let re = Regex::new(r#"^(.*?)\s+(\d+\.\d+\.\d+)\s+-\s+(.*?)$"#).unwrap();

//     if let Some(captures) = re.captures(input) {
//         let name = captures.get(1).map_or("", |m| m.as_str());
//         let version = captures.get(2).map_or("", |m| m.as_str());
//         let status = captures.get(3).map_or("", |m| m.as_str());

//         (name.to_string(), version.to_string(), status.to_string())
//     } else {
//         ("".to_string(), "".to_string(), "".to_string())
//     }
// }

// fn main() {
//     let strings = vec![
//         "Blender 4.1.0 - Alpha",
//         "Blender 4.0.0 - Beta",
//         "Blender 3.3.12 - Stable",
//         "Blender 3.3.12 - Stable",
//     ];

//     for s in strings {
//         let (name, version, status) = extract_parts(s);
//         println!("Name: {}, Version: {}, Status: {}", name, version, status);
//     }
// }


// fn extract_parts(input: &str) -> (&str, &str, &str) {
//     let parts: Vec<&str> = input.split('-').collect();
//     if parts.len() == 2 {
//         let part1 = parts[0].trim();
//         let rest = parts[1].trim();
//         if let Some(space_index) = rest.find(' ') {
//             let part2 = rest[..space_index].trim();
//             let part3 = rest[space_index + 1..].trim();
//             (part1, part2, part3)
//         } else {
//             (part1, rest, "")
//         }
//     } else {
//         ("", "", "")
//     }
// }

// fn main() {
//     let inputs = vec![
//         "Blender 4.1.0 - Alpha",
//         "Blender 4.0.0 - Beta",
//         "Blender 3.3.12 - Stable",
//         "Blender 3.3.12 - Stable",
//     ];

//     for input in inputs {
//         let (part1, part2, part3) = extract_parts(input);
//         println!("Input: {:?}", input);
//         println!("Part 1: {:?}", part1);
//         println!("Part 2: {:?}", part2);
//         println!("Part 3: {:?}", part3);
//     }
// }

// fn main() {
//     let strings = [
//         "Blender 4.1.0 - Alpha",
//         "Blender 4.0.0 - Beta",
//         "Blender 3.3.12 - Stable",
//         "Blender 3.3.12 - Stable",
//     ];

//     for s in &strings {
//         // Split the string by space characters and collect the parts into a vector of strings
//         let parts: Vec<&str> = s.split(' ').collect();

//         // Check if there are at least two parts in the split result
//         if parts.len() >= 2 {
//             // Extract the first part (index 0), which is "Blender"
//             let software_name = parts[0];
            
//             println!("Software: {}", software_name);
//         }
//     }
// }


// fn main() {
//     let strings = [
//         "Blender 4.1.0 - Alpha",
//         "Blender 4.0.0 - Beta",
//         "Blender 3.3.12 - Stable",
//         "Blender 3.3.12 - Stable",
//     ];

//     for s in &strings {
//         // Split the string by space characters and collect the parts into a vector of strings
//         let parts: Vec<&str> = s.split(' ').collect();

//         // Check if there are at least three parts in the split result
//         if parts.len() >= 3 {
//             // Extract the parts "Blender", "4.1.0", and "Alpha"
//             let software_name = parts[0];
//             let version = parts[1];
//             let status = parts[3]; // "Alpha" is at index 4 in this case
            
//             println!("Software: {}, Version: {}, Status: {}", software_name, version, status);
//         }
//     }
// }



// fn main() {
//     let strings = [
//         "blender-2.79-linux-glibc219-i686.tar.bz2",
//         "blender-3.6.5-windows-x64.zip",
//         "blender-3.6.5-macos-arm64.dmg",
//         "blender-3.6.5-linux-x64.tar.xz",
//         "blender-2.79a-macOS-10.6.zip",
//     ];

//     // for s in &strings {
//     //     // Split the string by '.' to separate the filename and extension
//     //     let parts: Vec<&str> = s.split('.').collect();

//     //     // Check if there are at least two parts in the split result
//     //     if parts.len() >= 2 {
//     //         // The last part after the last '.' is the file extension
//     //         let file_extension = parts.last().unwrap();

//     //         // The extension may also include the preceding part (e.g., "tar.bz2")
//     //         let file_extension_with_prefix = parts[parts.len() - 2..].join(".");

//     //         println!("File Extension: {}", file_extension_with_prefix);
//     //     }
//     // }
//     for s in &strings {
//         // Split the string by '.' to separate the filename and extension
//         let parts: Vec<&str> = s.split('.').collect();

//         // Check if there are at least two parts in the split result
//         if parts.len() >= 2 {
//             // The last part after the last '.' is the file extension
//             let file_extension = parts.last().unwrap();

//             // Check if the last part starts with a letter
//             if file_extension.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) {
//                 // Include the last two parts as the extension
//                 let file_extension_with_prefix = parts[parts.len() - 2..].join(".");
//                 println!("File Extension: {}", file_extension_with_prefix);
//             } else {
//                 // Use only the last part as the extension
//                 println!("File Extension: {}", file_extension);
//             }
//         }
//     }

// }


// fn main() {
//     let strings = [
//         "blender-2.79-linux-glibc219-i686.tar.bz2",
//         "blender-3.6.5-windows-x64.zip",
//         "blender-3.6.5-macos-arm64.dmg",
//         "blender-3.6.5-linux-x64.tar.xz",
//         "blender-2.79a-macOS-10.6.zip",
//     ];

//     for s in &strings {
//         // Split the string by '.' to separate the filename and extension
//         let parts: Vec<&str> = s.split('.').collect();

//         // Check if there are at least two parts in the split result
//         if parts.len() >= 2 {
//             // The last part after the last '.' is the file extension
//             let file_extension = parts.last().unwrap();
//             // Check if the file extension is "xz" or "bz2" and add "tar" prefix
//             let file_extension = if *file_extension == "xz" || *file_extension == "bz2" {
//                 format!("tar.{}", file_extension)
//             } else {
//                 file_extension.to_string()
//             };
            
//             println!("File Extension: {}", file_extension);
//         }
//     }
// }


fn main() {
    let arr = [1, 2, 3, 4, 5];
    let target = 3;

    if arr.contains(&target) {
        println!("The array contains {}.", target);
    } else {
        println!("The array does not contain {}.", target);
    }
}