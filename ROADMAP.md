# rustget — Roadmap

A Rust package manager (winget-style) built as a learning project. Each phase ships something runnable. No phase starts until the previous one has a working binary behind it — half-finished phases don't count as "learning."

Rule for using this doc: don't pre-read Rust concepts. Hit the wall, then go learn the specific thing that's blocking you, then come back.

---

## Phase 0 — Hello, Cargo

**Status:** done

```bash
cargo new rustget
cd rustget
cargo run
```

Output: `Welcome to rustget!`

Concepts touched: `cargo new` layout, `main.rs`, `println!`, build vs run.

---

## Phase 1 — CLI skeleton

**Status:** in progress

Wire up subcommands with `clap`'s derive API — don't hand-parse `std::env::args()`, it teaches you nothing you'll keep.

```
rustget search <name>
rustget install <name>
rustget remove <name>
rustget list
rustget --help
```

Structure to aim for:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search { name: String },
    Install { name: String },
    Remove { name: String },
    List,
}
```

**Done when:** `rustget --help` lists all four commands with descriptions, and each subcommand currently just prints a stub line ("Would install: firefox") — no real logic yet.

Concepts you'll hit: enums with data, `match`, `Result` vs `panic!` for bad input, derive macros.

Crate: `clap` (`features = ["derive"]`)

---

## Phase 2 — Package database (local, static)

**Status:** not started

Flat-file "repo" first — no server, no network. One JSON file per package:

```
repository/
├── firefox.json
├── vscode.json
└── rust.json
```

```json
{
  "name": "firefox",
  "version": "1.0.0",
  "url": "https://example.com/firefox.zip",
  "sha256": ""
}
```

Build a `Package` struct with `#[derive(Serialize, Deserialize)]`, load every `.json` in `repository/` at startup, and implement:

```bash
rustget search firefox   # substring match on name
rustget info firefox     # pretty-print the struct
```

**Done when:** `search` returns partial matches, `info` errors cleanly (not a panic) on an unknown package name.

Concepts: `serde`/`serde_json`, `std::fs::read_dir`, `Option`/`Result` chaining with `?`, deriving traits, basic error types (`thiserror` optional here, or hand-roll an enum).

Crates: `serde`, `serde_json`

---

## Phase 3 — Actually download something

**Status:** not started

```bash
rustget install firefox
```

should hit the `url` from the package's JSON and write bytes to disk, with a progress bar and a hard failure on non-200 responses (don't silently write an HTML error page to disk as if it were the package).

**Done when:** installing a real small file (e.g. a public test zip) produces a byte-identical file on disk, and a bad URL produces a clear error, not a corrupted download.

Concepts: async fundamentals just enough to call `reqwest`, streaming a response body to a file instead of buffering it all in memory, `?` across error types (this is usually where people first feel the pain of mismatched error types — that's the point where you learn `thiserror`/`anyhow`, not before).

Crates: `reqwest` (`features = ["stream"]`), `tokio`, `indicatif` (progress bar)

---

## Phase 4 — Install for real

**Status:** not started

Unzip into an install directory, track what's installed, support removal.

```bash
rustget install firefox
rustget remove firefox
rustget list
```

Keep a manifest — `~/.rustget/installed.json` or similar — mapping installed package → version → files it wrote. `remove` deletes exactly those files, not a glob guess.

**Done when:** `list` reflects reality after both install and remove, and removing a package that was never installed gives a clean error instead of deleting nothing silently or panicking.

Concepts: `zip` crate, `std::fs` (create_dir_all, copy, remove_file), platform-appropriate data dirs (`dirs` crate) instead of hardcoding `~`.

Crate: `zip`

---

## Phase 5 — Don't install malware by accident

**Status:** not started

Before this phase, `rustget` will happily install anything a URL serves. Fix that.

- Verify downloaded file's SHA-256 against the `sha256` field in the package JSON before extracting
- Refuse to install (with a clear message) on mismatch — don't just warn
- Add a `trusted_sources` allowlist for repo URLs

**Done when:** a deliberately corrupted/tampered download is rejected, not installed.

Concepts: `sha2`, hashing streams vs whole-buffer hashing, why you check the hash *before* you extract, not after.

Crate: `sha2`

---

## Phase 6 — Updates, versions, dependencies

**Status:** not started

```bash
rustget update              # refresh repo index
rustget upgrade firefox
rustget install firefox@1.2.0
rustget rollback firefox
```

Dependencies: a package can declare others it needs (`"depends": ["openssl", "zlib"]`). Start with the naive case — install all deps, no version conflict resolution — and only build a real resolver if you actually hit a conflict. Don't build `cargo`'s resolver on faith.

Concepts: semver parsing/comparison, keeping a rollback-capable history (old binaries or reinstall-from-index), basic graph traversal for the dependency list (a `Vec` and a loop is fine before it's a `HashMap` of `HashSet`s).

Crate: `semver`

---

## Phase 7 — Your own repository server

**Status:** not started

A small HTTP service that serves the package index instead of static local files.

```bash
GET /packages/firefox
GET /search?q=fire
POST /packages          # upload metadata
```

Backed by SQLite so the index isn't just JSON-on-disk anymore.

**Done when:** the Phase 2–6 client can point at `http://localhost:PORT` instead of a local folder and behave identically.

Concepts: `axum` routing, request/response serialization, a real (if small) SQL schema, running client and server as separate binaries in the same workspace.

Crates: `axum`, `sqlx` or `rusqlite`, `tokio`

---

## Phase 8 — The Rust you picked up sideways

Nothing to build here — it's the retro. By this point you'll have bumped into most of these without deciding to "study" them first:

- Ownership / borrowing (definitely, by Phase 3)
- Lifetimes (probably in Phase 4, holding onto path/string references)
- Traits (Phase 2 derives, Phase 6 if you build a resolver)
- Async/await + `tokio` (Phase 3 onward)
- `Arc`/`Mutex` or channels, if Phase 9's parallel downloads happens

Worth doing a deliberate pass through *The Rust Book* chapters on whichever of these still feel shaky — not to learn them cold, but to backfill the mental model now that you have working code to anchor it to.

---

## Phase 9 — Making it feel like a real tool

**Status:** stretch, pick based on interest

- Parallel downloads (`tokio::spawn` + a join set)
- Local download cache, keyed by hash
- Mirror fallback on failed download
- Config file (`~/.rustget/config.toml`) for repo URLs, install dir, etc.
- Structured logging instead of `println!` scattered everywhere
- Package signing (beyond hash verification — actual signatures)
- Cross-platform install paths (Windows/macOS/Linux)

---

## Stretch goals (post-1.0 ideas, not commitments)

- GUI frontend
- Self-updating `rustget`
- Private/authenticated repositories
- Sandboxed installs
- Automatic orphaned-dependency cleanup
- `rustget publish` for pushing packages to a repo

---

## Crate reference

| Purpose | Crate |
|---|---|
| CLI parsing | `clap` |
| Serialization | `serde`, `serde_json` |
| HTTP client | `reqwest` |
| HTTP server | `axum` |
| Async runtime | `tokio` |
| Hashing | `sha2` |
| Archives | `zip` |
| Versioning | `semver` |
| Progress bars | `indicatif` |
| Logging | `tracing` |
| Error handling | `thiserror` (library-style errors), `anyhow` (app-style errors) |
| Data directories | `dirs` |
| SQLite | `rusqlite` or `sqlx` |

---

## How to actually use this roadmap

Build the phase. When something in it requires a concept you don't have, stop and learn *that concept*, not the whole surrounding topic. Come back and finish the phase. Don't move on until the "done when" line for that phase is true — vague progress isn't progress.
