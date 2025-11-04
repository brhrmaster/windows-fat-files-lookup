use std::path::PathBuf;
use std::process::Command;
use log::{info, warn};
use winapi::um::fileapi::GetLogicalDrives;
use winapi::um::fileapi::GetDriveTypeA;
use std::ffi::CString;

// Drive type constants
const DRIVE_FIXED: u32 = 3;
const DRIVE_REMOVABLE: u32 = 2;
const DRIVE_REMOTE: u32 = 4;

pub fn get_available_disks() -> Vec<String> {
    info!("Enumerating available disks");
    let mut disks = Vec::new();
    
    unsafe {
        let drives = GetLogicalDrives();
        info!("Logical drives bitmask: 0x{:X}", drives);
        
        for i in 0..26 {
            if (drives & (1 << i)) != 0 {
                let drive_letter = (b'A' + i) as char;
                let drive_path = format!("{}:\\", drive_letter);
                let drive_path_c = CString::new(drive_path.clone()).unwrap();
                
                let drive_type = GetDriveTypeA(drive_path_c.as_ptr());
                info!("Drive {}: type {}", drive_letter, drive_type);
                
                // Include fixed drives, removable drives, and network drives
                if drive_type == DRIVE_FIXED || drive_type == DRIVE_REMOVABLE || drive_type == DRIVE_REMOTE {
                    disks.push(format!("{}:", drive_letter));
                    info!("Added drive {}: (type: {})", drive_letter, drive_type);
                } else {
                    warn!("Skipped drive {}: unsupported type {}", drive_letter, drive_type);
                }
            }
        }
    }
    
    disks.sort();
    info!("Found {} available disks: {:?}", disks.len(), disks);
    disks
}

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn get_file_icon(file_path: &str) -> &'static str {
    let extension = PathBuf::from(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();
    
    match extension.as_str() {
        "txt" | "md" | "log" => "📄",
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => "🖼️",
        "mp4" | "avi" | "mkv" | "mov" | "wmv" => "🎬",
        "mp3" | "wav" | "flac" | "aac" | "ogg" => "🎵",
        "zip" | "rar" | "7z" | "tar" | "gz" => "📦",
        "exe" | "msi" | "app" => "⚙️",
        "pdf" => "📕",
        "doc" | "docx" => "📘",
        "xls" | "xlsx" => "📗",
        "ppt" | "pptx" => "📙",
        "html" | "htm" | "css" | "js" | "json" | "xml" => "🌐",
        _ => "📄",
    }
}

pub fn open_in_explorer(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Opening Explorer for path: {}", path);
    
    // Convert the path to a PathBuf to get the parent directory
    let path_buf = PathBuf::from(path);
    
    // For files, open the parent directory and select the file
    // For folders, open the folder directly
    if path_buf.is_file() {
        // Open parent directory and select the file
        let parent = path_buf.parent()
            .ok_or("Could not get parent directory")?;
        
        let parent_str = parent.to_string_lossy();
        let file_name = path_buf.file_name()
            .ok_or("Could not get file name")?
            .to_string_lossy();
        
        info!("Opening parent directory: {} and selecting file: {}", parent_str, file_name);
        
        // Use explorer with /select parameter to select the file
        Command::new("explorer")
            .args(&["/select,", path])
            .spawn()?;
    } else {
        // Open the directory directly
        info!("Opening directory: {}", path);
        
        Command::new("explorer")
            .arg(path)
            .spawn()?;
    }
    
    info!("Explorer opened successfully");
    Ok(())
}



