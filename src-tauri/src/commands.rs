use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use sysinfo::Disks;

#[tauri::command]
pub fn get_disk_space() -> Result<u64, String> {
    let disks = Disks::new_with_refreshed_list();
    let disk = disks.list().first().ok_or("No disks found".to_string())?;
    Ok(disk.available_space())
}

#[tauri::command]
pub fn hash_file(path: String) -> Result<String, String> {
    let mut file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}
