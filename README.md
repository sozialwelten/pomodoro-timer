# 🍅 Pomodoro Timer CLI

Ein einfacher Pomodoro-Timer für die Kommandozeile, geschrieben in Rust.

## Features

- Work Sessions (25 Minuten)
- Short Break (5 Minuten)
- Long Break (15 Minuten)
- Custom Timer für beliebige Zeitspannen
- Visueller Countdown im Terminal
- Akustischer Alarm bei Ablauf

## Installation & Verwendung

```bash
# Projekt klonen
git clone https://github.com/DEIN-USERNAME/pomodoro-timer.git
cd pomodoro-timer

# Kompilieren und ausführen
cargo run
```

## Was man bei diesem Projekt lernt

### Rust Grundlagen
- **Pattern Matching** mit `match` für Menü-Navigation
- **User Input** mit `std::io` verarbeiten
- **String Handling** und Parsing
- **Error Handling** mit `Result` und `unwrap()`
- **Threading** mit `std::thread::sleep()` für Timer-Funktionalität

### Cargo - Rusts Build-System & Package Manager

**Cargo** ist das offizielle Build-Tool für Rust und macht folgendes:
- **Projekt-Management**: Erstellt neue Projekte mit `cargo new`
- **Dependency-Management**: Verwaltet externe Bibliotheken (Crates)
- **Kompilierung**: `cargo build` kompiliert den Code
- **Ausführung**: `cargo run` kompiliert UND führt das Programm aus
- **Testing**: `cargo test` führt Tests aus

Cargo verwendet die `Cargo.toml`-Datei als Konfiguration (ähnlich wie `package.json` in Node.js).

### Warum heißt die Datei `main.rs`?

In Rust-Projekten ist **`main.rs` die Konvention** für den Einstiegspunkt:

- `src/main.rs` ist die **Standard-Datei** für ausführbare Programme (Binaries)
- Cargo sucht automatisch nach `src/main.rs` als Entry Point
- Die Datei enthält die `main()`-Funktion, die beim Programmstart aufgerufen wird
- Der Name des Projekts wird in `Cargo.toml` definiert, nicht durch den Dateinamen

**Alternative:** Für Bibliotheken (Libraries) würde man stattdessen `src/lib.rs` verwenden.

### Wie funktioniert die Kompilierung?

```bash
# Debug-Build (schneller kompilieren, langsamer laufen)
cargo build

# Release-Build (optimiert für Performance)
cargo build --release

# Direkt kompilieren und ausführen
cargo run

# Kompilierte Dateien löschen
cargo clean
```

Die kompilierten Binaries landen im `target/`-Verzeichnis:
- `target/debug/pomodoro-timer` - Debug-Version
- `target/release/pomodoro-timer` - Optimierte Version

**Wichtig:** Das `target/`-Verzeichnis wird NICHT in Git eingecheckt (`.gitignore`), da es sehr groß ist und jeder das Projekt selbst kompilieren kann.

## Technische Details

- **Sprache**: Rust (Edition 2021)
- **Dependencies**: Nur Standard Library
- **Plattform**: Cross-platform (Linux, macOS, Windows)

## Mögliche Erweiterungen

- [ ] Persistente Statistiken (wie viele Pomodoros heute?)
- [ ] Sound-Datei für Alarm statt System-Beep
- [ ] Config-Datei für eigene Zeiteinstellungen
- [ ] Desktop-Notifications
- [ ] Timer-Historie

## Autor

Michael Karbacher

## Lizenz

GPL-3.0 - siehe [LICENSE](LICENSE) Datei für Details.

---

**Lernprojekt** zum Üben von Rust-Grundlagen und Cargo-Workflow.
