# Rust for C++-studenter

Et undervisningsopplegg til TDT4102 Prosedyre- og objektorientert programmering, som sammenlikner C++ og Rust. Innholdet er tilgjengelig i lesbart format på [isakbjugn.github.io/tdt4102-rust](https://isakbjugn.github.io/tdt4102-rust/).

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
