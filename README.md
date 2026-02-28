# fagdag-cpp

Et undervisningsprosjekt som sammenlikner C++ og Rust, bygd som en [mdBook](https://rust-lang.github.io/mdBook/).

## Kom i gang

### Krav

- [Rust toolchain](https://rustup.rs/) (for mdBook og Rust-eksempel)
- C++20-kompilator (clang++ eller g++)

### Installer mdBook og mdbook-lang

```bash
cargo install mdbook --version 0.4.51
cargo install mdbook-lang
```

Dersom du har en nyere versjon av mdBook installert globalt, kan du kopiere 0.4-binæren til et eget navn:

```bash
cp ~/.cargo/bin/mdbook ~/.cargo/bin/mdbook-04
cargo install mdbook  # installer nyeste tilbake
```

### Vis boken

```bash
mdbook-lang server start   # start C++-kompileringsserver
mdbook-04 serve --open     # eller mdbook serve om du bare har 0.4.x
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
