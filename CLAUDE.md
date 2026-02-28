# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Brukeren skriver alltid på norsk (bokmål). Svar på norsk (bokmål).

## Prosjektoversikt

Undervisningsprosjekt som sammenligner minnehåndtering i C++ og Rust, bygd som en mdBook for TDT4102-kurset (NTNU). Boka er skrevet på norsk. Innhold ligger i markdown under `src/`, med tilhørende kode i `cpp/` og `rust/`.

## Bygg- og kjørekommandoer

```bash
# Kjør mdBook lokalt (installer med: cargo install mdbook)
mdbook serve --open

# Bygg mdBook
mdbook build

# Bygg og kjør Rust-eksempler
cd rust && cargo run

# Bygg og kjør C++-eksempler (krever C++20-kompilator)
clang++ -std=c++20 cpp/minnehandtering/main.cpp -o main && ./main
```

## Arkitektur

- **`src/`** — mdBook-kilder i markdown. `SUMMARY.md` definerer innholdslisten. Hvert konsept har `teori.md` (teori) og `oppgaver.md` (oppgaver).
- **`rust/`** — Cargo-prosjekt (edition 2024). Moduler speiler bokstrukturen (f.eks. `src/minnehandtering/`).
- **`cpp/`** — Frittstående C++-eksempelfiler, én per konseptmappe.
- **`book.toml`** — mdBook-konfigurasjon. Byggutdata går til `book/`.

## Kodeankere

Rust-kildefiler bruker `// ANCHOR:` og `// ANCHOR_END:`-kommentarer for å markere koderegioner som blir inkludert i mdBook via `{{#rustdoc_include}}`-direktiv. Ta vare på disse ankerne ved redigering av Rust-kode, og hold ankernavnene i synk med markdown-filene som refererer til dem.

## Utrulling

GitHub Actions (`.github/workflows/mdbook.yml`) bygger mdBook og deployer til GitHub Pages ved hvert push til `main`.
