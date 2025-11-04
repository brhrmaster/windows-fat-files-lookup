# Fat Folder Discovery Log Monitor
# This script monitors the log file in real-time

param(
    [string]$LogFile = "fat-folder-discovery.log",
    [int]$RefreshInterval = 1
)

Write-Host "=== Fat Folder Discovery Log Monitor ===" -ForegroundColor Green
Write-Host "Monitoring log file: $LogFile" -ForegroundColor Yellow
Write-Host "Refresh interval: $RefreshInterval seconds" -ForegroundColor Yellow
Write-Host "Press Ctrl+C to stop monitoring" -ForegroundColor Red
Write-Host ""

# Check if log file exists
if (-not (Test-Path $LogFile)) {
    Write-Host "Log file '$LogFile' not found. Waiting for application to create it..." -ForegroundColor Yellow
    while (-not (Test-Path $LogFile)) {
        Start-Sleep -Seconds 1
    }
    Write-Host "Log file created! Starting monitoring..." -ForegroundColor Green
}

# Get initial file size
$lastSize = (Get-Item $LogFile).Length

try {
    while ($true) {
        # Check if file size changed
        $currentSize = (Get-Item $LogFile).Length
        
        if ($currentSize -gt $lastSize) {
            # Read new content
            $content = Get-Content $LogFile -Tail 20
            Clear-Host
            Write-Host "=== Fat Folder Discovery Log Monitor ===" -ForegroundColor Green
            Write-Host "File: $LogFile | Size: $currentSize bytes | Last updated: $(Get-Date)" -ForegroundColor Cyan
            Write-Host ""
            
            # Display recent log entries
            foreach ($line in $content) {
                if ($line -match "ERROR") {
                    Write-Host $line -ForegroundColor Red
                } elseif ($line -match "WARN") {
                    Write-Host $line -ForegroundColor Yellow
                } elseif ($line -match "INFO") {
                    Write-Host $line -ForegroundColor White
                } else {
                    Write-Host $line -ForegroundColor Gray
                }
            }
            
            $lastSize = $currentSize
        }
        
        Start-Sleep -Seconds $RefreshInterval
    }
} catch {
    Write-Host "Monitoring stopped." -ForegroundColor Red
}
