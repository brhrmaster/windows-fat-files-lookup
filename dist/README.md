# Fat Folder Discovery

A Windows utility for finding and managing large files and folders on your disk storage.

## Quick Start

1. Run `fat-folder-discovery.exe` by double-clicking
2. Select a drive or enter a specific folder path
3. Click "Scan" to begin analysis
4. Review the results in real-time

No installation required - the application is portable.

## System Requirements

- Windows 10 or Windows 11
- Read access to directories you want to scan
- No special hardware requirements

## Basic Usage

1. **Scanning a Drive**:
   - Select a drive from the dropdown menu
   - Click "Scan" to begin
   - Watch results appear in real-time

2. **Scanning a Specific Folder**:
   - Enter the folder path in the path field
   - Click "Scan" to analyze
   - The appropriate drive will be selected automatically

3. **Understanding Results**:
   - Left panel shows largest folders
   - Right panel shows largest files
   - Click any item to open its location in Explorer
   - File sizes are shown in human-readable format (KB, MB, GB)

## Additional Features

- Use sliders to adjust how many items to display (5-20)
- Click the refresh button to update drive list
- Stop a scan at any time with the Stop button
- Type-specific icons for different file types

## Important Notes

- The application automatically skips system directories for safety
- Log files are automatically cleaned up on exit
- Each file/folder entry shows:
  - Name and full path
  - Size in readable format
  - Type-specific icon for files

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

## Getting Help

For issues, feature requests, or contributions, please visit:
https://github.com/brhrmaster/windows-fat-files-lookup

---

**Fat Folder Discovery** - Find your disk space usage quickly and efficiently