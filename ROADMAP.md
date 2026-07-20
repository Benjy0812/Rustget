# rustget ROADMAP

A project-driven roadmap for building a custom Rust package manager inspired by tools like **winget**.

> **Goal:** Learn Rust by building a real-world project. Learn concepts when the project requires them instead of studying everything upfront.

---

# Phase 0 — Project Setup

**Status:** ✅ Completed

## Objective

Create the first working `rustget` project.

### Learn
- Install Rust
- Install Cargo
- Create a new project
- Run and build a project
- Basic Rust syntax

### Build

```bash
cargo new rustget
cd rustget
cargo run
```

Make the program print:

```text
Welcome to rustget!
```

---

# Phase 1 — Build the CLI

**Status:** 🚧 In Progress

## Objective

Create the command structure.

Commands:

```bash
rustget search
rustget install
rustget remove
rustget list
rustget --help
```

### Learn (only when needed)

- Functions
- Structs
- Enums
- Modules
- Error handling

### Crates

- clap

### Milestone

Running:

```bash
rustget --help
```

should display all available commands.

---

# Phase 2 — Package Database

## Objective

Teach rustget what packages exist.

Example structure:

```
repository/
├── firefox.json
├── vscode.json
└── rust.json
```

Example package:

```json
{
    "name": "firefox",
    "version": "1.0.0",
    "url": "https://example.com/firefox.zip"
}
```

### Features

- Read JSON
- Search packages
- Display package information

Commands:

```bash
rustget search firefox
rustget info firefox
```

### Learn

- File I/O
- JSON
- Serialization

### Crates

- serde
- serde_json

---

# Phase 3 — Download Packages

## Objective

Make this command work:

```bash
rustget install firefox
```

### Features

- Download files
- Save downloads
- Show progress
- Handle errors

### Learn

- HTTP
- Networking
- File paths

### Crates

- reqwest

---

# Phase 4 — Install Packages

## Objective

Install software onto the computer.

### Features

- Extract archives
- Copy files
- Store installed package information
- Remove packages

Commands:

```bash
rustget install firefox
rustget remove firefox
rustget list
```

### Learn

- Directories
- File systems
- ZIP archives

---

# Phase 5 — Security

## Objective

Make installs safe.

### Features

- SHA-256 verification
- Hash checking
- Better error messages
- Trusted repositories

### Learn

- Hashing
- Cryptography basics
- Secure software practices

---

# Phase 6 — Updates & Dependencies

## Updates

```bash
rustget update
rustget upgrade firefox
```

## Versions

```bash
rustget install firefox@1.2.0
rustget rollback firefox
```

## Dependencies

Example:

```
my-app
├── openssl
├── sqlite
└── zlib
```

### Learn

- Dependency resolution
- Versioning

---

# Phase 7 — Repository Server

## Objective

Create your own package repository.

### Features

- Package uploads
- Package metadata
- Search API
- User repositories

### Learn

- REST APIs
- Web servers
- Databases

Possible database:

- SQLite

---

# Phase 8 — Advanced Rust

Learn these naturally as the project grows.

Topics:

- Ownership
- Borrowing
- Lifetimes
- Traits
- Async programming
- Threads
- Performance optimization

Don't study these first.
Learn them when the project requires them.

---

# Phase 9 — Professional Features

Add things real package managers have.

### Features

- Parallel downloads
- Download cache
- Mirrors
- Configuration files
- Logging
- Package signing
- Repository priorities
- Cross-platform support
- Plugin system

---

# Stretch Goals

Ideas for future versions:

- GUI frontend
- Self-updating rustget
- Private repositories
- Sandboxed installs
- Automatic dependency cleanup
- Package analytics
- AI-assisted package search
- Package publishing CLI

---

# Recommended Crates

| Purpose | Crate |
|---------|-------|
| CLI | clap |
| JSON | serde |
| JSON Parsing | serde_json |
| HTTP | reqwest |
| Hashing | sha2 |
| ZIP Archives | zip |
| Async Runtime | tokio |
| Logging | tracing |

---

# Project Philosophy

✅ Learn by building.

If you need to understand:

- Ownership → Learn ownership.
- Traits → Learn traits.
- Lifetimes → Learn lifetimes.

Don't try to master Rust before starting.

Build first.
Learn when you get stuck.
Repeat.

That's how real software engineers grow.
