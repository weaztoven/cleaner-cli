# 🧹 cleaner-cli

Ein schnelles und leichtes CLI-Tool in Rust, um unnötige Ordner wie `node_modules`, `target`, `.cache` und `dist` zu finden und zu bereinigen.

---

### 🚀 Features

- 🔍 **Schneller Scan:** Rekursive Suche nach Entwickler-Müll.
- 📊 **Speicher-Analyse:** Zeigt den freizugebenden Speicherplatz in KB/MB/GB an.
- 🛡️ **Sicherheitsmodus:** Standardmäßig wird nur simuliert. Gelöscht wird erst mit dem Flag `--execute`.
- ⚡ **Rust Performance:** Maximale Geschwindigkeit bei minimaler Ressourcennutzung.

---

### 📦 Installation & Nutzung

1. **Repository klonen:**
   ```bash
   git clone [https://github.com/weaztoven/cleaner-cli.git](https://github.com/weaztoven/cleaner-cli.git)
   cd cleaner-cli