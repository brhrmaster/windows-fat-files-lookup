@echo off
echo Testing Fat Folder Discovery Log Cleanup...
echo.

echo 1. Starting application...
start /wait fat-folder-discovery.exe

echo 2. Checking if log file exists after normal close...
if exist fat-folder-discovery.log (
    echo Log file still exists - cleanup may not have worked
    echo Log file size:
    dir fat-folder-discovery.log
) else (
    echo Log file successfully cleaned up!
)

echo.
echo Test completed.
pause
