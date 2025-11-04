# Fat Folder Discovery

A Windows desktop application built in Rust that helps you identify the largest files and folders on your disk storage. Perfect for disk cleanup and storage management.

## Features

### 🎯 Core Functionality
- **Disk Selection**: Choose from available storage drives (C:, D:, etc.)
- **Custom Path Scanning**: Scan specific directories or entire disk roots
- **Real-Time Results**: See files and folders appear as they're discovered
- **Size-Based Sorting**: Results automatically sorted by size (largest first)
- **Configurable Limits**: Display 5-20 largest files and folders

### 📊 User Interface
- **Fixed Window Size**: Consistent 800x600 pixel window
- **Two-Column Layout**: 
  - Left: "Fat Folders" with folder icons and sizes
  - Right: "Fat Files" with file type icons and sizes
- **Real-Time Updates**: Watch results populate during scanning
- **Size Display**: Human-readable sizes (KB, MB, GB)
- **Status Bar**: Live scanning progress and configuration info

### 🔧 Technical Features
- **Full Disk Scanning**: Traverses entire disk regardless of display limits
- **Asynchronous Processing**: Non-blocking UI during scans
- **File Type Icons**: Visual indicators for different file types
- **Logging System**: Comprehensive logging for monitoring and debugging
- **Windows Integration**: Native Windows disk enumeration

## Screenshots

The application features:
- Clean, professional interface with dark theme
- Real-time scanning progress indicators
- Two-column results display with proper sizing
- Status bar showing current configuration and progress

## Requirements

- **Operating System**: Windows 10/11
- **Build Tools**: Visual Studio Build Tools with C++ workload
- **Rust**: Latest stable version

## Installation

### Prerequisites
1. Install [Rust](https://rustup.rs/)
2. Install Visual Studio Build Tools with "Desktop development with C++" workload

### Build Instructions

#### **Method 1: Automated Build (Recommended)**
```bash
# Clone the repository
git clone <repository-url>
cd fat-folder-discovery

# Build and automatically copy to dist folder
cargo build --release
# ✅ Executable automatically copied to dist/fat-folder-discovery.exe
```

#### **Method 2: Batch Script (Windows)**
```bash
# Run the automated build script
.\build_and_distribute.bat
# ✅ Builds, copies executable and README to dist folder
```

#### **Method 3: Manual Build**
```bash
# Build the application
cargo build --release

# Run the application
cargo run --release
```

## Usage

### Basic Operation
1. **Select Disk**: Choose your target drive from the dropdown
2. **Set Limits**: Adjust file and folder display limits (5-20)
3. **Start Scan**: Click the scan button to begin analysis
4. **Monitor Progress**: Watch real-time results populate
5. **Review Results**: Examine largest files and folders

### Advanced Usage
- **Custom Paths**: Enter specific directory paths for targeted scanning
- **Log Monitoring**: Use `monitor_log.ps1` to watch scanning progress
- **Full Disk Analysis**: Leave path empty to scan entire disk root

## File Structure

```
fat-folder-discovery/
├── src/
│   ├── main.rs           # Main application logic and UI
│   ├── disk_scanner.rs   # File system scanning engine
│   ├── file_utils.rs     # Disk enumeration and utilities
│   ├── ui_components.rs  # Reusable UI components
│   └── logger.rs         # Custom logging implementation
├── Cargo.toml            # Dependencies and project config
├── monitor_log.ps1       # Log monitoring script
└── README.md             # This file
```

## Dependencies

- **eframe/egui**: Modern Rust GUI framework
- **walkdir**: Efficient directory traversal
- **winapi**: Windows API integration
- **log**: Comprehensive logging system
- **tokio**: Asynchronous runtime

## Performance

- **Efficient Scanning**: Uses binary heaps for top-N selection
- **Memory Optimized**: Maintains only display-limited results in memory
- **Real-Time Updates**: Results appear every 50 folders / 500 files
- **Non-Blocking UI**: Scanning runs in separate thread

## Logging

The application creates `fat-folder-discovery.log` with detailed information:
- Application startup and configuration
- Disk enumeration results
- Scanning progress and statistics
- Error handling and recovery

**Automatic Cleanup**: Log files are automatically deleted when the application closes normally (via Close button or window close). This keeps the distribution clean and prevents log accumulation.

Monitor logs in real-time:
```powershell
.\monitor_log.ps1
```

## Technical Details

### Architecture
- **GUI Framework**: eframe/egui for modern Rust desktop UI
- **Scanning Engine**: Custom implementation with walkdir
- **Data Structures**: BinaryHeap for efficient top-N maintenance
- **Threading**: std::thread for non-blocking operations
- **Communication**: mpsc channels for UI updates

### Window Layout
- **Total Size**: 800x600 pixels (fixed)
- **Top Panel**: ~60px (controls and configuration)
- **Main Content**: 800x500px (two 395px columns)
- **Bottom Panel**: ~40px (status and actions)

## Contributing

This is a focused desktop utility for Windows disk analysis. The codebase follows Rust best practices with clean separation of concerns.

## License

[Add your license information here]

---

**Fat Folder Discovery** - Find your disk space hogs quickly and efficiently! 🚀