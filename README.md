# 🦀 Rustget

**Rustget** is an experimental package manager written in Rust, inspired by Microsoft's Winget.

The project serves as both a learning exercise and an attempt to build a lightweight, extensible package manager with support for custom JSON package manifests. While Rustget starts as a learning project, the long-term goal is to create a useful package manager that anyone can use.

---

## ✨ Planned Features

- 📦 Install packages
- 🗑️ Remove packages
- 🔍 Search repositories
- 📋 List installed packages
- 🔄 Update installed packages
- 📄 JSON-based package manifests
- 🌐 Custom package repositories
- 🔐 SHA-256 package verification
- ⚡ Fast command-line interface
- 🧩 Extensible repository format

---

## 📂 Package Manifests

Rustget uses JSON manifests to describe how packages should be installed.

Example:

```json
{
  "name": "firefox",
  "version": "1.0.0",
  "url": "https://example.com/firefox.zip",
  "hash": "sha256-hash"
}
```

The repository will include official package manifests, but users can also create and host their own repositories.

---

## 🎯 Project Goals

Rustget is being built to gain practical experience with:

- Rust
- Systems programming
- Command-line application development
- Networking
- File systems
- Package management
- Software architecture
- Secure software development

Rather than learning every Rust concept upfront, the project is developed incrementally—learning new concepts as they're needed.

---

## 🗺️ Roadmap

See **ROADMAP.md** for the full development roadmap and planned milestones.

---

## 💡 Ideas & Feedback

Have an idea? Open an issue!

Whether it's:

- A feature request
- A bug report
- A package request
- A design suggestion
- A question
- Or just a random "what if..."

Feel free to create an issue.

No idea is too small, too weird, or too ambitious. Sometimes the simplest or strangest ideas become the best features.

---

## 🚧 Project Status

**Current Stage:** Early Development

Rustget is in the beginning stages of development. Features will be added incrementally as the project grows.

---

## 📄 License

This project is licensed under the **MIT License**.
