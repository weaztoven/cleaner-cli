# 🧹 cleaner-cli

A fast and lightweight Rust CLI tool to find and clean up heavy, unnecessary development folders like node_modules, target, .cache, and dist.

---

### 🚀 Features

- 🔍 Fast Scanning: Recursively searches for deep developer junk across directories.
- 📊 Storage Analysis: Displays total disk space to be reclaimed in KB/MB/GB.
- 🛡️ Safety Mode: Runs in dry-run/simulation mode by default. No files are removed without the --execute flag.
- ⚡ Rust Performance: Blazing fast execution with minimal memory footprint.

---

### 📦 Installation & Usage

1. Clone the Repository:
   ```bash 
   git clone https://github.com/weaztoven/cleaner-cli.git
   cd cleaner-cli

3. Install via Cargo:
   ```bash 
   cargo install --path .
