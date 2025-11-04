@echo off
echo Building Fat Folder Discovery...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo Build successful! Copying executable to dist...
    if not exist dist mkdir dist
    copy target\release\fat-folder-discovery.exe dist\
    copy README.md dist\
    echo.
    echo ✅ Build complete! Executable available in dist\ folder
    echo 📁 Files in dist:
    dir dist
) else (
    echo ❌ Build failed!
    exit /b 1
)
