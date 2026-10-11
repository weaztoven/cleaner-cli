use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Hauptverzeichnis, das durchsucht und bereinigt werden soll
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Tatsächlich löschen (falls nicht angegeben, wird nur gescannt und die Größe angezeigt)
    #[arg(short, long)]
    execute: bool,
}

// Ziel-Ordner, die für die Bereinigung vorgesehen sind
const TARGET_DIRS: &[&str] = &[
    "node_modules",
    "target",
    ".cache",
    "dist",
    "build",
    ".next",
    "__pycache__",
];

fn main() {
    let args = Args::parse();
    let root_path = Path::new(&args.path);

    if !root_path.exists() {
        println!("{}", "Fehler: Das angegebene Verzeichnis wurde nicht gefunden!".red());
        return;
    }

    let absolute_path = fs::canonicalize(root_path).unwrap_or_else(|_| root_path.to_path_buf());
    
    // Windows \\?\ Präfixreinigung
    let path_str = absolute_path.to_string_lossy();
    let clean_path = if path_str.starts_with(r"\\?\") {
        &path_str[4..]
    } else {
        &path_str
    };

    println!(
        "{} Suche läuft in: {}",
        "🔍".bold(),
        clean_path.cyan()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} Ordner werden durchsucht... ({elapsed})")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut found_targets: Vec<PathBuf> = Vec::new();

    // Verzeichnisse rekursiv durchsuchen
    for entry in WalkDir::new(root_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            if let Some(dir_name) = entry.file_name().to_str() {
                if TARGET_DIRS.contains(&dir_name) {
                    found_targets.push(entry.path().to_path_buf());
                }
            }
        }
    }

    pb.finish_and_clear();

    if found_targets.is_empty() {
        println!("{}", "✨ Keine zu bereinigenden Zielordner gefunden!".green());
        return;
    }

    println!("\nGefundene Zielordner:");
    let mut total_size: u64 = 0;

    for path in &found_targets {
        let dir_size = calculate_dir_size(path);
        total_size += dir_size;
        println!(
            "  - {} ({})",
            path.display().to_string().yellow(),
            format_size(dir_size).bright_black()
        );
    }

    println!(
        "\nInsgesamt freizugebender Speicherplatz: {}",
        format_size(total_size).green().bold()
    );

    if args.execute {
        println!("\n{} Bereinigung wird gestartet...", "🚀".bold());
        for path in found_targets {
            match fs::remove_dir_all(&path) {
                Ok(_) => println!("  Gelöscht: {}", path.display()),
                Err(e) => eprintln!("  Konnte nicht gelöscht werden {}: {}", path.display(), e),
            }
        }
        println!("{}", "✨ Bereinigung abgeschlossen!".green().bold());
    } else {
        println!(
            "\n{}",
            "ℹ️ Löschvorgang simuliert. Um tatsächlich zu löschen, füge das Flag '--execute' oder '-e' hinzu."
                .cyan()
        );
    }
}

// Hilfsfunktion zur Berechnung der Ordnergröße
fn calculate_dir_size(path: &Path) -> u64 {
    let mut size = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                size += metadata.len();
            }
        }
    }
    size
}

// Konvertiert Dateigrößen in ein lesbares Format (Byte -> KB/MB/GB)
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} Bytes", bytes)
    }
}
