# fagdag-cpp

Et undervisningsprosjekt som samanliknar C++ og Rust, bygd som ein [mdBook](https://rust-lang.github.io/mdBook/).

## Kom i gang

### Krav

- [Rust toolchain](https://rustup.rs/) (for mdBook og Rust-eksempel)
- C++20-kompilator (clang++ eller g++)

### Installer mdBook

```bash
cargo install mdbook
```

### Vis boka

```bash
mdbook serve --open
```

### Bygg og køyr kodeeksempla

**C++:**

```bash
clang++ -std=c++20 code/minnehandtering/main.cpp -o main && ./main
```

**Rust:**

```bash
cd rust && cargo run
```

## Prosjektstruktur

```
├── book.toml              # mdBook-konfigurasjon
├── src/                   # mdBook markdown-kjelder
│   ├── SUMMARY.md
│   ├── introduksjon.md
│   └── minnehandtering/
├── code/                  # Kjørbar demokode
│   └── minnehandtering/
└── rust/                  # Rust Cargo-prosjekt
    ├── Cargo.toml
    └── src/
```
