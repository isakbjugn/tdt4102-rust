# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Educational project comparing C++ and Rust memory management, built as an mdBook for the TDT4102 course (NTNU). The book is written in Norwegian. Content lives in markdown under `src/`, with companion code in `cpp/` and `rust/`.

## Build & Run Commands

```bash
# Serve the mdBook locally (install with: cargo install mdbook)
mdbook serve --open

# Build the mdBook
mdbook build

# Build and run Rust examples
cd rust && cargo run

# Build and run C++ examples (requires C++20 compiler)
clang++ -std=c++20 cpp/minnehandtering/main.cpp -o main && ./main
```

## Architecture

- **`src/`** — mdBook markdown sources. `SUMMARY.md` defines the table of contents. Each concept has a `teori.md` (theory) and `oppgaver.md` (exercises).
- **`rust/`** — Cargo project (edition 2024). Modules mirror the book's concept structure (e.g., `src/minnehandtering/`).
- **`cpp/`** — Standalone C++ example files, one per concept directory.
- **`book.toml`** — mdBook configuration. Build output goes to `book/`.

## Code Anchors

Rust source files use `// ANCHOR:` and `// ANCHOR_END:` comments to mark code regions that get embedded into the mdBook via `{{#rustdoc_include}}` directives. Preserve these anchors when editing Rust code, and keep the anchor names in sync with the markdown files that reference them.

## Deployment

GitHub Actions (`.github/workflows/mdbook.yml`) builds the mdBook and deploys to GitHub Pages on every push to `main`.
