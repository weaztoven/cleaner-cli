use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

const TARGET_DIRS: &[&str] = &["node_modules", "target", ".cache", "dist"];

struct CleanupItem {
    path: PathBuf,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let execute_mode = args.iter().any(|arg| arg == "--execute");
    let export_report = args.iter().any(|arg| arg == "--report");

    let current_dir = std::env::current_dir().expect("Failed to get current directory!");
    
    println!("🧹 cleaner-cli - Development Junk Cleaner\n");
    println!("🔍 Scanning directory: {}", current_dir.display());
    
    if !execute_mode {
        println!("🛡️  SAFETY MODE (Dry-run): No files will be deleted. Use '--execute' to delete.\n");
    } else {
        println!("⚠️  WARNING: --execute is active. Found folders will be processed for deletion!\n");
    }

    let start_time = Instant::now();
    let mut found_items = Vec::new();

    search_targets(&current_dir, &mut found_items);

    if found_items.is_empty() {
        println!("✨ No target folders found!");
        return;
    }

    let mut total_size: u64 = 0;
    let mut processed_count = 0;

    for item in &found_items {
        let size = dir_size(&item.path);
        total_size += size;
        processed_count += 1;

        let size_mb = size as f64 / 1024.0 / 1024.0;
        println!("📁 Found: {} ({:.2} MB)", item.path.display(), size_mb);
    }

    let total_mb = total_size as f64 / 1024.0 / 1024.0;
    let total_gb = total_mb / 1024.0;

    // --- ONAY MEKANİZMASI ---
    if execute_mode {
        print!("\n⚠️  Are you sure you want to permanently delete these {} folders ({:.2} MB)? [y/N]: ", processed_count, total_mb);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();

        if choice != "y" && choice != "yes" {
            println!("❌ Operation cancelled by user.");
            return;
        }

        println!("\n🗑️ Starting deletion process...");
        for item in &found_items {
            match fs::remove_dir_all(&item.path) {
                Ok(_) => println!("   └─ Deleted: {}", item.path.display()),
                Err(e) => eprintln!("   └─ Failed to delete {}: {}", item.path.display(), e),
            }
        }
    }

    let elapsed = start_time.elapsed();

    // --- REPORT SUMMARY ---
    println!("\n{}", "═".repeat(50));
    println!("📊 CLEANUP EXECUTION REPORT");
    println!("{}", "═".repeat(50));
    println!("⏱️  Duration      : {:.2?}", elapsed);
    println!("📦 Target Folders: {}", processed_count);
    if total_gb >= 1.0 {
        println!("💾 Reclaimable Space: {:.2} GB ({:.2} MB)", total_gb, total_mb);
    } else {
        println!("💾 Reclaimable Space: {:.2} MB", total_mb);
    }
    println!("⚙️  Mode          : {}", if execute_mode { "Executed (Deleted)" } else { "Dry-run (Simulated)" });
    println!("{}", "═".repeat(50));

    if export_report {
        let report_content = format!(
            "Cleaner-CLI Report\nTime: {:.2?}\nFolders: {}\nTotal Size: {:.2} MB\nMode: {}\n",
            elapsed, processed_count, total_mb, if execute_mode { "Executed" } else { "Dry-run" }
        );
        if let Err(e) = fs::write("cleaner-report.txt", report_content) {
            eprintln!("❌ Failed to save report file: {}", e);
        } else {
            println!("📄 Report successfully saved to 'cleaner-report.txt'");
        }
    }
}

fn search_targets(dir: &Path, found: &mut Vec<CleanupItem>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if TARGET_DIRS.contains(&dir_name) {
                        found.push(CleanupItem { path: path.clone() });
                        continue; 
                    }
                }
                search_targets(&path, found);
            }
        }
    }
}

fn dir_size(path: &Path) -> u64 {
    let mut size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Ok(metadata) = p.metadata() {
                    size += metadata.len();
                }
            } else if p.is_dir() {
                size += dir_size(&p);
            }
        }
    }
    size
}