<div align="center">

# 🦀 Rustget

**An experimental package manager written in Rust — inspired by Winget.**

![status](https://img.shields.io/badge/status-early%20development-orange)
![license](https://img.shields.io/badge/license-MIT-blue)
![made with](https://img.shields.io/badge/made%20with-Rust-orange?logo=rust)

</div>

---

Rustget is both a learning exercise and an attempt to build a lightweight, extensible package manager with support for custom JSON package manifests. It starts as a learning project — but the long-term goal is a genuinely useful package manager that anyone can use.

---

## ✨ Planned Features

| | |
|---|---|
| 📦 Install packages | 🗑️ Remove packages |
| 🔍 Search repositories | 📋 List installed packages |
| 🔄 Update installed packages | 📄 JSON-based package manifests |
| 🌐 Custom package repositories | 🔐 SHA-256 package verification |
| ⚡ Fast command-line interface | 🧩 Extensible repository format |

---

## 📂 Package Manifests

Rustget uses JSON manifests to describe how packages should be installed.

```json
{
  "name": "firefox",
  "version": "1.0.0",
  "url": "https://example.com/firefox.zip",
  "hash": "sha256-hash"
}
```

The repository will include official package manifests, but anyone can create and host their own.

---

## 🎯 Project Goals

Rustget is being built to gain practical experience with:

`Rust` · `Systems programming` · `CLI development` · `Networking` · `File systems` · `Package management` · `Software architecture` · `Secure software development`

Rather than learning every Rust concept upfront, the project is developed incrementally — new concepts are picked up as they're needed.

---

## 🗺️ Roadmap

See **[ROADMAP.md](./ROADMAP.md)** for the full development roadmap and planned milestones.

---

## 📝 Notes

> **[CONTRIBUTING.md](./CONTRIBUTING.md)** is a personal reference doc for keeping package manifests consistent (schema, description style, verification policy) — not a public contribution guide. This project isn't accepting outside contributions yet.

---

## 💡 Ideas & Feedback

Have an idea? Open an issue — a feature request, bug report, package request, design suggestion, question, or just a random *"what if..."*

No idea is too small, too weird, or too ambitious. Sometimes the simplest or strangest ideas become the best features.

---

## 🚧 Project Status

**Current stage:** Early Development — features are being added incrementally as the project grows.

---

## 📄 License

Licensed under the **MIT License**.
