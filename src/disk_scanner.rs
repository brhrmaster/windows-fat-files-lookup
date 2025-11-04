use std::path::{Path, PathBuf};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};
use log::{info, warn};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub name: String,
    pub path: String,
    pub size: u64,
}

impl PartialEq for ScanResult {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl Eq for ScanResult {}

impl PartialOrd for ScanResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScanResult {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for max heap (largest first)
        other.size.cmp(&self.size)
    }
}

#[derive(Debug)]
pub struct ScanResults {
    pub folders: Vec<ScanResult>,
    pub files: Vec<ScanResult>,
}

pub struct DiskScanner {
    root_path: PathBuf,
    file_limit: usize,
    folder_limit: usize,
    should_stop: Arc<Mutex<bool>>,
    result_sender: Option<mpsc::Sender<ScanResults>>,
}

impl DiskScanner {
    
    pub fn new_with_sender(root_path: PathBuf, file_limit: usize, folder_limit: usize, sender: mpsc::Sender<ScanResults>) -> Self {
        Self {
            root_path,
            file_limit,
            folder_limit,
            should_stop: Arc::new(Mutex::new(false)),
            result_sender: Some(sender),
        }
    }
    
    fn send_incremental_results(&self, folders: &BinaryHeap<ScanResult>, files: &BinaryHeap<ScanResult>) {
        if let Some(sender) = &self.result_sender {
            let mut folder_vec: Vec<ScanResult> = folders.iter().cloned().collect();
            folder_vec.sort_by(|a, b| b.size.cmp(&a.size));
            
            let mut file_vec: Vec<ScanResult> = files.iter().cloned().collect();
            file_vec.sort_by(|a, b| b.size.cmp(&a.size));
            
            let results = ScanResults {
                folders: folder_vec,
                files: file_vec,
            };
            
            let _ = sender.send(results);
        }
    }
    
    pub fn scan(&mut self) -> Result<ScanResults, Box<dyn std::error::Error>> {
        info!("Starting disk scan of: {}", self.root_path.display());
        *self.should_stop.lock().unwrap() = false;
        
        let mut folder_heap: BinaryHeap<ScanResult> = BinaryHeap::new();
        let mut file_heap: BinaryHeap<ScanResult> = BinaryHeap::new();
        let mut total_files_scanned = 0u64;
        let mut total_folders_scanned = 0u64;
        
        info!("Walking directory tree starting from: {}", self.root_path.display());
        info!("Note: Scanning entire disk, limits only affect display (top {} folders, {} files)", 
            self.folder_limit, self.file_limit);
        
        // Walk through the directory tree
        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .max_depth(1000) // Prevent infinite recursion
        {
            // Check if we should stop
            if *self.should_stop.lock().unwrap() {
                warn!("Scan stopped by user request");
                break;
            }
            
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    warn!("Failed to access entry: {}", e);
                    continue; // Skip inaccessible files/folders
                }
            };
            
            let path = entry.path();
            
            // Skip system directories and files
            if self.should_skip_path(path) {
                continue;
            }
            
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    warn!("Failed to get metadata for {}: {}", path.display(), e);
                    continue;
                }
            };
            
            if metadata.is_dir() {
                total_folders_scanned += 1;
                if total_folders_scanned % 1000 == 0 {
                    info!("Scanned {} folders so far...", total_folders_scanned);
                }
                
                let folder_size = self.calculate_folder_size(path)?;
                let folder_result = ScanResult {
                    name: path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Root")
                        .to_string(),
                    path: path.to_string_lossy().to_string(),
                    size: folder_size,
                };
                
                folder_heap.push(folder_result);
                
                // Keep only the largest folders for display, but continue scanning
                if folder_heap.len() > self.folder_limit {
                    folder_heap.pop();
                }
                
                // Send incremental update every 50 folders (more frequent for better real-time feel)
                if total_folders_scanned % 50 == 0 {
                    self.send_incremental_results(&folder_heap, &file_heap);
                }
            } else if metadata.is_file() {
                total_files_scanned += 1;
                if total_files_scanned % 10000 == 0 {
                    info!("Scanned {} files so far...", total_files_scanned);
                }
                
                let file_result = ScanResult {
                    name: path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    path: path.to_string_lossy().to_string(),
                    size: metadata.len(),
                };
                
                file_heap.push(file_result);
                
                // Keep only the largest files for display, but continue scanning
                if file_heap.len() > self.file_limit {
                    file_heap.pop();
                }
                
                // Send incremental update every 500 files (more frequent for better real-time feel)
                if total_files_scanned % 500 == 0 {
                    self.send_incremental_results(&folder_heap, &file_heap);
                }
            }
        }
        
        info!("Scan completed: {} files, {} folders processed", total_files_scanned, total_folders_scanned);
        
        // Send final results
        self.send_incremental_results(&folder_heap, &file_heap);
        
        // Convert heaps to sorted vectors
        let mut folders: Vec<ScanResult> = folder_heap.into_vec();
        folders.sort_by(|a, b| b.size.cmp(&a.size));
        
        let mut files: Vec<ScanResult> = file_heap.into_vec();
        files.sort_by(|a, b| b.size.cmp(&a.size));
        
        info!("Returning top {} folders and {} files", folders.len(), files.len());
        
        Ok(ScanResults { folders, files })
    }
    
    fn calculate_folder_size(&self, path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
        let mut total_size = 0u64;
        
        for entry in WalkDir::new(path)
            .follow_links(false)
            .max_depth(100) // Prevent deep recursion
        {
            // Check if we should stop
            if *self.should_stop.lock().unwrap() {
                break;
            }
            
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            
            if entry.file_type().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }
        
        Ok(total_size)
    }
    
    fn should_skip_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();
        
        // Skip system directories
        let skip_patterns = [
            "$recycle.bin",
            "system volume information",
            "windows\\system32",
            "windows\\winsxs",
            "windows\\temp",
            "programdata\\microsoft\\windows\\wer",
            "programdata\\microsoft\\windows\\caches",
            "users\\default\\appdata\\local\\temp",
            "users\\default\\appdata\\local\\microsoft\\windows\\inetcache",
        ];
        
        skip_patterns.iter().any(|pattern| path_str.contains(pattern))
    }
    
    pub fn stop(&self) {
        *self.should_stop.lock().unwrap() = true;
    }
}

impl Drop for DiskScanner {
    fn drop(&mut self) {
        self.stop();
    }
}


