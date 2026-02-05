# fagdag-cpp

Et undervisningsprosjekt for å utforske grunnleggende C++-konsepter, med fokus på hvordan disse sammenlikner med moderne språk som Rust.

## Formål

Prosjektet tar for seg sentrale konsepter i C++:

- **Minnehåndtering** - manuell allokering med `new`/`delete` vs. Rusts eierskap-modell
- **Smartpekere** - `unique_ptr`, `shared_ptr`, `weak_ptr` vs. Rusts `Box`, `Rc`, `Arc`
- **RAII** - Resource Acquisition Is Initialization, et mønster begge språk bygger på
- **Livstider** - implisitte i C++, eksplisitte i Rust
- **Move-semantikk** - `std::move` i C++ vs. move-by-default i Rust

## Oppsett

### Krav

- C++20-kompilator (clang++ eller g++)
- VS Code (anbefalt)

### VS Code-utvidelser

**clangd** (anbefalt) fremfor Microsofts "C/C++"-utvidelse:

| | clangd | Microsoft C/C++ |
|---|--------|-----------------|
| Hastighet | Raskere indeksering | Tregere på store prosjekter |
| Presisjon | Bruker ekte kompilator-frontend | Egen parsing, kan avvike |
| Feilmeldinger | Identiske med kompilering | Kan gi andre resultater |
| Refaktorering | God støtte | Begrenset |

clangd bruker samme Clang-frontend som kompilatoren din, så feilmeldinger og autofullføring matcher nøyaktig det du ser ved kompilering.

### Kompilering

```bash
clang++ -std=c++20 -Wall -Wextra -o main main.cpp
```

## Sammenlikning med Rust

| Konsept | C++ | Rust |
|---------|-----|------|
| Manuell minnehåndtering | `new`/`delete` | Ikke tilgjengelig |
| Unik eier | `std::unique_ptr<T>` | `Box<T>` |
| Delt eierskap | `std::shared_ptr<T>` | `Rc<T>` / `Arc<T>` |
| Trådsikker deling | `std::shared_ptr<T>` + mutex | `Arc<T>` + `Mutex<T>` |
| Null-sikkerhet | Ingen garanti | `Option<T>` |
| Minnefeil | Udefinert oppførsel | Kompileringsfeil |
