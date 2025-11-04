use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc;
use log::{info, warn, error};

mod disk_scanner;
mod file_utils;
mod logger;

use disk_scanner::{DiskScanner, ScanResult, ScanResults};
use file_utils::{get_available_disks, format_size, get_file_icon, open_in_explorer};
use logger::{init_logging, cleanup_logs};

#[derive(Default)]
pub struct FatFolderDiscoveryApp {
    // Disk and path selection
    selected_disk: String,
    custom_path: String,
    available_disks: Vec<String>,
    
    // Scanning configuration
    file_limit: usize,
    folder_limit: usize,
    
    // Results
    fat_folders: Vec<ScanResult>,
    fat_files: Vec<ScanResult>,
    
    // UI state
    is_scanning: bool,
    scan_progress: f32,
    scan_status: String,
    
    // Scanner
    scanner: Option<Arc<Mutex<DiskScanner>>>,
    
    // Channel communication
    scan_receiver: Option<mpsc::Receiver<ScanResults>>,
    scan_sender: Option<mpsc::Sender<()>>,
    scan_completion_receiver: Option<mpsc::Receiver<()>>,
    
}

impl FatFolderDiscoveryApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        info!("Initializing FatFolderDiscoveryApp");
        let mut app = Self::default();
        app.file_limit = 10;
        app.folder_limit = 10;
        
        info!("Refreshing available disks");
        // Initialize available disks
        app.refresh_disks();
        
        app.scan_status = format!("Ready to scan root of {}", app.selected_disk);
        
        info!("Application initialization completed");
        app
    }
    
    fn refresh_disks(&mut self) {
        info!("Starting disk refresh");
        self.scan_status = "Refreshing disk list...".to_string();
        self.available_disks = get_available_disks();
        info!("Found {} available disks: {:?}", self.available_disks.len(), self.available_disks);
        
        if !self.available_disks.is_empty() && self.selected_disk.is_empty() {
            self.selected_disk = self.available_disks[0].clone();
            info!("Auto-selected disk: {}", self.selected_disk);
        }
        self.scan_status = format!("Ready to scan root of {}", self.selected_disk);
        info!("Disk refresh completed");
    }
    
    fn start_scan(&mut self) {
        if self.is_scanning {
            warn!("Scan already in progress, ignoring start request");
            return;
        }
        
        info!("Starting scan process");
        
        // Validate inputs
        if self.selected_disk.is_empty() {
            error!("No disk selected for scanning");
            self.scan_status = "Error: No disk selected".to_string();
            return;
        }
        
        let scan_path = if self.custom_path.is_empty() {
            format!("{}\\", self.selected_disk)
        } else {
            self.custom_path.clone()
        };
        
        info!("Scan path: {}", scan_path);
        
        // Validate path exists
        if !scan_path.is_empty() && !std::path::Path::new(&scan_path).exists() {
            error!("Scan path does not exist: {}", scan_path);
            self.scan_status = "Error: Path does not exist".to_string();
            return;
        }
        
        info!("Starting scan with limits: {} files, {} folders", self.file_limit, self.folder_limit);
        
        self.is_scanning = true;
        self.scan_progress = 0.0;
        self.scan_status = "Initializing scan...".to_string();
        self.fat_folders.clear();
        self.fat_files.clear();
        
        // Create channels for communication
        let (result_sender, result_receiver) = mpsc::channel();
        let (stop_sender, _stop_receiver) = mpsc::channel();
        let (completion_sender, completion_receiver) = mpsc::channel();
        
        self.scan_receiver = Some(result_receiver);
        self.scan_sender = Some(stop_sender);
        self.scan_completion_receiver = Some(completion_receiver);
        
        // Start scanning in background thread
        let file_limit = self.file_limit;
        let folder_limit = self.folder_limit;
        
        self.scan_status = "Scanning files and folders...".to_string();
        info!("Spawning background scan thread");
        
        std::thread::spawn(move || {
            info!("Background scan thread started");
            let mut scanner = DiskScanner::new_with_sender(
                PathBuf::from(scan_path),
                file_limit,
                folder_limit,
                result_sender,
            );
            
            match scanner.scan() {
                Ok(results) => {
                    info!("Scan completed successfully: {} folders, {} files", 
                        results.folders.len(), results.files.len());
                    let _ = completion_sender.send(());
                }
                Err(e) => {
                    error!("Scan failed: {}", e);
                    let _ = completion_sender.send(());
                }
            }
            info!("Background scan thread finished");
        });
        
        info!("Scan process initiated successfully");
    }
    
    fn stop_scan(&mut self) {
        info!("Stopping scan process");
        self.is_scanning = false;
        self.scan_status = "Scan stopped".to_string();
        self.scanner = None;
        self.scan_receiver = None;
        self.scan_sender = None;
        self.scan_completion_receiver = None;
        info!("Scan process stopped");
    }
    
    fn check_scan_results(&mut self) {
        // Check for incremental results
        if let Some(receiver) = &self.scan_receiver {
            while let Ok(results) = receiver.try_recv() {
                info!("Received incremental scan results: {} folders, {} files", 
                    results.folders.len(), results.files.len());
                
                // Update the lists with new results
                info!("Before update: {} folders, {} files in UI", self.fat_folders.len(), self.fat_files.len());
                self.fat_folders = results.folders;
                self.fat_files = results.files;
                info!("After update: {} folders, {} files in UI", self.fat_folders.len(), self.fat_files.len());
                
                // Update status with current counts and show real-time progress
                self.scan_status = format!("Scanning... Found {} folders, {} files", 
                    self.fat_folders.len(), self.fat_files.len());
                
                info!("Updated UI with {} folders and {} files", 
                    self.fat_folders.len(), self.fat_files.len());
                
                // Force immediate UI update
                info!("UI should now show updated results");
                
                // Request immediate repaint to show results
                // This will be handled by the main update loop
            }
        }
        
        // Check for completion signal
        if let Some(completion_receiver) = &self.scan_completion_receiver {
            if let Ok(_) = completion_receiver.try_recv() {
                info!("Scan completion signal received");
                self.scan_status = format!("Scan completed: {} folders, {} files found", 
                    self.fat_folders.len(), self.fat_files.len());
                self.scan_progress = 1.0;
                self.is_scanning = false;
                self.scan_receiver = None;
                self.scan_sender = None;
                self.scan_completion_receiver = None;
                
                info!("Scan process completed and cleaned up");
            }
        }
    }
}

impl Clone for FatFolderDiscoveryApp {
    fn clone(&self) -> Self {
        Self {
            selected_disk: self.selected_disk.clone(),
            custom_path: self.custom_path.clone(),
            available_disks: self.available_disks.clone(),
            file_limit: self.file_limit,
            folder_limit: self.folder_limit,
            fat_folders: self.fat_folders.clone(),
            fat_files: self.fat_files.clone(),
            is_scanning: self.is_scanning,
            scan_progress: self.scan_progress,
            scan_status: self.scan_status.clone(),
            scanner: None, // Don't clone scanner
            scan_receiver: None, // Don't clone channels
            scan_sender: None, // Don't clone channels
            scan_completion_receiver: None, // Don't clone channels
        }
    }
}

impl Drop for FatFolderDiscoveryApp {
    fn drop(&mut self) {
        info!("Application shutting down, cleaning up logs...");
        cleanup_logs();
    }
}

impl eframe::App for FatFolderDiscoveryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for scan results
        self.check_scan_results();
        
        // Top panel with controls
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Disk selector
                ui.label("Disk:");
                let old_disk = self.selected_disk.clone();
                egui::ComboBox::from_id_salt("disk_selector")
                    .selected_text(&self.selected_disk)
                    .show_ui(ui, |ui| {
                        for disk in &self.available_disks {
                            ui.selectable_value(&mut self.selected_disk, disk.clone(), disk);
                        }
                    });
                
                // Provide feedback when disk changes
                if old_disk != self.selected_disk && !self.selected_disk.is_empty() {
                    self.scan_status = format!("Selected disk: {}", self.selected_disk);
                }
                
                ui.separator();
                
                // Path input
                ui.label("Path:");
                let old_path = self.custom_path.clone();
                ui.text_edit_singleline(&mut self.custom_path);
                
                // Provide feedback when path changes
                if old_path != self.custom_path {
                    if self.custom_path.is_empty() {
                        self.scan_status = format!("Path cleared - will scan root of {}", self.selected_disk);
                    } else {
                        self.scan_status = format!("Custom path: {}", self.custom_path);
                    }
                }
                
                // Auto-select disk from path
                if !self.custom_path.is_empty() {
                    if let Some(disk_letter) = self.custom_path.chars().next() {
                        let disk_str = format!("{}:", disk_letter.to_uppercase());
                        if self.available_disks.contains(&disk_str) {
                            self.selected_disk = disk_str;
                        }
                    }
                }
                
                // Scan button
                let scan_button_text = if self.is_scanning { "Stop" } else { "Scan" };
                if ui.button(scan_button_text).clicked() {
                    if self.is_scanning {
                        self.stop_scan();
                    } else {
                        self.start_scan();
                    }
                }
                
                // Refresh disks button
                if ui.button("🔄").clicked() {
                    self.scan_status = "Refreshing disk list...".to_string();
                    self.refresh_disks();
                }
            });
            
            // Configuration row
            ui.horizontal(|ui| {
                ui.label("File limit:");
                let old_file_limit = self.file_limit;
                ui.add(egui::Slider::new(&mut self.file_limit, 5..=20));
                
                ui.label("Folder limit:");
                let old_folder_limit = self.folder_limit;
                ui.add(egui::Slider::new(&mut self.folder_limit, 5..=20));
                
                // Provide feedback when limits change
                if old_file_limit != self.file_limit || old_folder_limit != self.folder_limit {
                    self.scan_status = format!("Limits updated: {} files, {} folders", 
                        self.file_limit, self.folder_limit);
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(&self.scan_status);
                    if self.is_scanning {
                        // Animate progress bar
                        self.scan_progress = (self.scan_progress + 0.01) % 1.0;
                        ui.add(egui::ProgressBar::new(self.scan_progress));
                    }
                });
            });
        });
        
        // Main content area - Fixed size: 800x500 (600 - 60 top - 40 bottom)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left column - Fat Folders (Fixed: 395px width, 500px height)
                ui.push_id("folders_column", |ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(395.0, 500.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            ui.heading("📁 Fat Folders");
                            ui.separator();
                            
                            // Fixed scroll area height: 500 - 40 (heading + separator) = 460px
                            egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .max_height(460.0)
                                .show(ui, |ui| {
                                    if self.fat_folders.is_empty() && self.is_scanning {
                                        ui.label("Scanning folders...");
                                    }
                                    for folder in &self.fat_folders {
                                        ui.horizontal(|ui| {
                                            ui.label("📁");
                                            ui.label(format!("[{}]", format_size(folder.size)));
                                            ui.vertical(|ui| {
                                                ui.label(&folder.name);
                                                ui.label(egui::RichText::new(&folder.path).size(10.0).weak());
                                            });
                                        });
                                        
                                        // Make the entire row clickable
                                        if ui.add(egui::Button::new("").fill(egui::Color32::TRANSPARENT)).clicked() {
                                            info!("Clicked on folder: {}", folder.path);
                                            if let Err(e) = open_in_explorer(&folder.path) {
                                                error!("Failed to open Explorer for folder {}: {}", folder.path, e);
                                                self.scan_status = format!("Error opening Explorer: {}", e);
                                            }
                                        }
                                        ui.separator();
                                    }
                                });
                        }
                    );
                });
                
                ui.separator();
                
                // Right column - Fat Files (Fixed: 395px width, 500px height)
                ui.push_id("files_column", |ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(395.0, 500.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            ui.heading("📄 Fat Files");
                            ui.separator();
                            
                            // Fixed scroll area height: 500 - 40 (heading + separator) = 460px
                            egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .max_height(460.0)
                                .show(ui, |ui| {
                                    if self.fat_files.is_empty() && self.is_scanning {
                                        ui.label("Scanning files...");
                                    }
                                    for file in &self.fat_files {
                                        ui.horizontal(|ui| {
                                            ui.label(get_file_icon(&file.path));
                                            ui.label(format!("[{}]", format_size(file.size)));
                                            ui.vertical(|ui| {
                                                ui.label(&file.name);
                                                ui.label(egui::RichText::new(&file.path).size(10.0).weak());
                                            });
                                        });
                                        
                                        // Make the entire row clickable
                                        if ui.add(egui::Button::new("").fill(egui::Color32::TRANSPARENT)).clicked() {
                                            info!("Clicked on file: {}", file.path);
                                            if let Err(e) = open_in_explorer(&file.path) {
                                                error!("Failed to open Explorer for file {}: {}", file.path, e);
                                                self.scan_status = format!("Error opening Explorer: {}", e);
                                            }
                                        }
                                        ui.separator();
                                    }
                                });
                        }
                    );
                });
            });
        });
        
        // Status bar panel
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Status information
                ui.label("Status:");
                ui.label(egui::RichText::new(&self.scan_status).color(egui::Color32::LIGHT_BLUE));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Show current configuration
                    ui.label(format!("Limits: {} files, {} folders", self.file_limit, self.folder_limit));
                    ui.separator();
                    ui.label(format!("Disk: {}", self.selected_disk));
                });
            });
        });
        
        // Bottom panel with close button
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Close").clicked() {
                    self.stop_scan();
                    info!("User requested application close, cleaning up...");
                    cleanup_logs();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
        
        // Request repaint for progress updates
        if self.is_scanning {
            // More frequent repaints during scanning for real-time updates
            ctx.request_repaint_after(Duration::from_millis(16)); // ~60 FPS
        }
        
        // Always request repaint to ensure UI updates are visible
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    if let Err(e) = init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
    }
    
    // Set up signal handler for cleanup on termination
    ctrlc::set_handler(|| {
        println!("Application terminated, cleaning up logs...");
        cleanup_logs();
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");
    
    info!("Starting Fat Folder Discovery application");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0])
            .with_max_inner_size([800.0, 600.0])
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };
    
    info!("Creating application window with size 800x600");
    
    // Run the application
    let result = eframe::run_native(
        "Fat Folder Discovery",
        options,
        Box::new(|cc| {
            info!("Application window created, initializing app state");
            Ok(Box::new(FatFolderDiscoveryApp::new(cc)))
        }),
    );
    
    // Clean up logs when application exits (regardless of how it exits)
    info!("Application exiting, cleaning up logs...");
    cleanup_logs();
    
    result
}
