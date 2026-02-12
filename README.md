# fagdag-cpp

Et undervisningsprosjekt som sammenlikner C++ og Rust, bygd som en [mdBook](https://rust-lang.github.io/mdBook/).

## Kom i gang

### Krav

- [Rust toolchain](https://rustup.rs/) (for mdBook og Rust-eksempel)
- C++20-kompilator (clang++ eller g++)

### Installer mdBook

```bash
cargo install mdbook
```

### Vis boken

```bash
mdbook serve --open
```

### Bygg og kjør kodeeksemplene

**C++:**

```bash
clang++ -std=c++20 cpp/minnehandtering/main.cpp -o main && ./main
```

**Rust:**

```bash
cd rust && cargo run
```

## Prosjektstruktur

```
├── book.toml              # mdBook-konfigurasjon
├── src/                   # mdBook markdown-kilder
│   ├── SUMMARY.md
│   ├── introduksjon.md
│   └── minnehandtering/
├── cpp/                   # C++-kode
│   └── minnehandtering/
└── rust/                  # Rust Cargo-prosjekt
    ├── Cargo.toml
    └── src/
```
