<div align="center">

# 🧹 cleaner-cli

**A blazing-fast, lightweight CLI tool written in Rust to reclaim disk space by hunting down developer junk.**

[![Crates.io](https://img.shields.io/crates/v/cleaner-cli.svg?style=flat-square&color=orange)](https://crates.io/crates/cleaner-cli)
[![Crates.io Downloads](https://img.shields.io/crates/d/cleaner-cli.svg?style=flat-square)](https://crates.io/crates/cleaner-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)

[Features](#-key-features) • [Installation](#-installation) • [Usage](#-usage--examples) • [Target Directories](#%EF%B8%8F-target-directories) • [License](#-license)

---

</div>

## ✨ Overview

Every developer's disk eventually fills up with abandoned `node_modules`, heavy Rust `target` build artifacts, `.cache`, and `dist` folders scattered across the filesystem. 

**`cleaner-cli`** searches through your workspace, calculates the exact disk space you can save, and gives you safe, interactive control to wipe the junk away.

---

## 🚀 Key Features

- ⚡ **Blazing Fast**: Powered by Rust for high-speed recursive file scanning with minimal RAM usage.
- 🛡️ **Safe by Default (Dry-Run)**: Always operates in simulation mode unless explicitly told to delete. No accidental data loss!
- ⚠️ **Interactive Confirmation**: Prompts for explicit `(y/N)` user confirmation before purging any files.
- 📊 **Detailed Storage Analysis**: Displays exact disk space to be reclaimed per directory and in total.
- 📝 **Report Exporting**: Generate clean summary reports (`cleaner-report.txt`) for audit logs or tracking.

---

## 📦 Installation

### Option 1: Cargo (Recommended)

If you have Rust installed, simply run:

```bash
cargo install cleaner-cli

```

### Option 2: From Source

```bash
# Clone the repository
git clone https://github.com/weaztoven/cleaner-cli.git
cd cleaner-cli

# Build and install locally
cargo install --path . --force

```

---

## 💡 Usage & Examples

### 1. Safe Scan (Default / Simulation Mode)

Scans your current directory recursively and lists all junk folders without deleting anything.

```bash
cleaner-cli

```

### 2. Execute & Clean

Scans for junk, asks for confirmation, and permanently removes the selected folders upon approval.

```bash
cleaner-cli --execute

```

### 3. Generate Summary Report

Exports a detailed report file (`cleaner-report.txt`) alongside your scan or deletion.

```bash
# Export report in dry-run mode
cleaner-cli --report

# Export report during active deletion
cleaner-cli --execute --report

```

---

## 🛠️ Target Directories

`cleaner-cli` automatically targets heavy build artifacts and caches, including:

| Folder | Ecosystem |
| --- | --- |
| `node_modules` | Node.js / JavaScript / TypeScript |
| `target` | Rust / Cargo build artifacts |
| `dist` / `build` | Web bundlers (Vite, Webpack, Next.js) |
| `.cache` | General package manager & tool caches |

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!

Feel free to check out the [Issues page](https://github.com/weaztoven/cleaner-cli/issues) if you want to request features or report bugs.

---

## 📄 License

Distributed under the **MIT License**. See `LICENSE` for more information.

Made with ❤️ and 🦀 Rust by [weaztoven](https://github.com/weaztoven)