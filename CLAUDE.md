# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Brukeren skriver alltid på norsk. Svar på norsk.

## Prosjektoversikt

Undervisningsprosjekt som samanliknar minnehandtering i C++ og Rust, bygd som ein mdBook for TDT4102-kurset (NTNU). Boka er skriven på norsk. Innhald ligg i markdown under `src/`, med tilhøyrande kode i `cpp/` og `rust/`.

## Bygg- og køyrkommandoar

```bash
# Køyr mdBook lokalt (installer med: cargo install mdbook)
mdbook serve --open

# Bygg mdBook
mdbook build

# Bygg og køyr Rust-eksempel
cd rust && cargo run

# Bygg og køyr C++-eksempel (krev C++20-kompilator)
clang++ -std=c++20 cpp/minnehandtering/main.cpp -o main && ./main
```

## Arkitektur

- **`src/`** — mdBook-kjelder i markdown. `SUMMARY.md` definerer innhaldslista. Kvart konsept har `teori.md` (teori) og `oppgaver.md` (oppgåver).
- **`rust/`** — Cargo-prosjekt (edition 2024). Modular speglar bokstrukturen (t.d. `src/minnehandtering/`).
- **`cpp/`** — Frittståande C++-eksempelfiler, éin per konseptmappe.
- **`book.toml`** — mdBook-konfigurasjon. Byggutdata går til `book/`.

## Kodeankre

Rust-kjeldefiler brukar `// ANCHOR:` og `// ANCHOR_END:`-kommentarar for å markere koderegionar som blir inkluderte i mdBook via `{{#rustdoc_include}}`-direktiv. Ta vare på desse ankra ved redigering av Rust-kode, og hald ankernamna i synk med markdown-filene som refererer til dei.

## Utrulling

GitHub Actions (`.github/workflows/mdbook.yml`) byggjer mdBook og deployer til GitHub Pages ved kvart push til `main`.
