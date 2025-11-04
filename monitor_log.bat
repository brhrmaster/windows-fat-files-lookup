@echo off
echo Starting Fat Folder Discovery Log Monitor...
echo.
echo This will monitor the log file in real-time.
echo Press Ctrl+C to stop monitoring.
echo.

powershell -ExecutionPolicy Bypass -File "monitor_log.ps1"

pause
