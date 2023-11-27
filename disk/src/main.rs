use sysinfo::{System, SystemExt, DiskExt};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let mut system = System::new_all();
    // system.refresh_disks_list();

    for disk in system.disks() {
        // println!("{:?}", disk);
        println!("{}", disk.mount_point().display());
        // println!("Disk type: {:?}", disk.get_type());
        // println!("Total space: {} bytes", disk.get_total_space());
        // println!("Available space: {} bytes", disk.get_available_space());
        let abc = format!("{}\\blenderbaseapps", disk.mount_point().to_string_lossy());
        println!("{abc}");
        let path_obj = Path::new(&abc);
        println!("{}", path_obj.exists())
        // // Check if a specific path exists on the disk
        // let path_to_check = format!("{}/path/to/check", disk.get_mount_point().to_str().unwrap_or(""));
        // if fs::metadata(&path_to_check).is_ok() {
        //     println!("Path {} exists on this disk.", path_to_check);
        // } else {
        //     println!("Path {} does not exist on this disk.", path_to_check);
        // }

        // println!("------------------------");
    }
}