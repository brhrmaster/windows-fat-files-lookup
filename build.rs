use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to re-run this build script if the executable changes
    println!("cargo:rerun-if-changed=target/release/fat-folder-discovery.exe");
    
    // Only run this in release builds
    if std::env::var("PROFILE").unwrap_or_default() == "release" {
        copy_executable_to_dist();
        copy_readme_to_dist();
    }
}

fn copy_executable_to_dist() {
    let exe_path = "target/release/fat-folder-discovery.exe";
    let dist_dir = "dist";
    let dist_exe = format!("{}/fat-folder-discovery.exe", dist_dir);
    
    // Create dist directory if it doesn't exist
    if !Path::new(dist_dir).exists() {
        if let Err(e) = fs::create_dir(dist_dir) {
            eprintln!("Warning: Could not create dist directory: {}", e);
            return;
        }
        println!("cargo:warning=Created dist directory");
    }
    
    // Copy the executable
    if Path::new(exe_path).exists() {
        if let Err(e) = fs::copy(exe_path, &dist_exe) {
            eprintln!("Warning: Could not copy executable to dist: {}", e);
        } else {
            println!("cargo:warning=✅ Executable copied to dist/fat-folder-discovery.exe");
        }
    } else {
        eprintln!("Warning: Executable not found at {}", exe_path);
    }
}

fn copy_readme_to_dist() {
    let readme_path = "README.md";
    let dist_readme = "dist/README.md";
    
    if Path::new(readme_path).exists() {
        if let Err(e) = fs::copy(readme_path, dist_readme) {
            eprintln!("Warning: Could not copy README to dist: {}", e);
        } else {
            println!("cargo:warning=📄 README.md copied to dist/");
        }
    }
}
