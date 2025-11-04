use log::{Level, LevelFilter, Metadata, Record, info};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;

pub struct FileLogger {
    file: Mutex<std::fs::File>,
}

impl FileLogger {
    pub fn new(log_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;
        
        Ok(FileLogger {
            file: Mutex::new(file),
        })
    }
    
    fn get_timestamp() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("{}", now)
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Self::get_timestamp();
            let log_entry = format!(
                "[{}] {} - {}: {}\n",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            );
            
            if let Ok(mut file) = self.file.lock() {
                let _ = file.write_all(log_entry.as_bytes());
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let logger = FileLogger::new("fat-folder-discovery.log")?;
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(LevelFilter::Info);
    
    info!("=== Fat Folder Discovery Application Started ===");
    info!("Logging initialized successfully");
    
    Ok(())
}

pub fn cleanup_logs() {
    let log_file_path = "fat-folder-discovery.log";
    
    if Path::new(log_file_path).exists() {
        match std::fs::remove_file(log_file_path) {
            Ok(_) => {
                // Use println! instead of log::info! since we're shutting down
                println!("Log file cleaned up successfully");
            }
            Err(e) => {
                eprintln!("Warning: Could not delete log file: {}", e);
            }
        }
    }
}
