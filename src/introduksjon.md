# Introduksjon

Et undervisningsprosjekt for å utforske grunnleggende C++-konsepter, med fokus på hvordan disse sammenlikner med moderne språk som Rust.

## Konsepter

Prosjektet tar for seg sentrale konsepter i C++:

- **Minnehåndtering** - manuell [allokering](./ordliste.md#allokering) med `new`/`delete` vs. Rusts [eierskap-modell](./ordliste.md#eierskap)
- **Smartpekere** - `unique_ptr`, `shared_ptr`, `weak_ptr` vs. Rusts `Box`, `Rc`, `Arc`
- **RAII** - Resource Acquisition Is Initialization, et mønster begge språk bygger på
- **Livstider** - implisitte i C++, eksplisitte i Rust
- **Move-semantikk** - `std::move` i C++ vs. move-by-default i Rust

## Sammenlikning

| Konsept | C++ | Rust |
|---------|-----|------|
| Manuell minnehåndtering | `new`/`delete` | Ikke tilgjengelig |
| Unik eier | `std::unique_ptr<T>` | `Box<T>` |
| Delt eierskap | `std::shared_ptr<T>` | `Rc<T>` / `Arc<T>` |
| Trådsikker deling | `std::shared_ptr<T>` + mutex | `Arc<T>` + `Mutex<T>` |
| Null-sikkerhet | Ingen garanti | `Option<T>` |
| Minnefeil | [Udefinert oppførsel](./ordliste.md#udefinert-oppforsel) | [Kompileringsfeil](./ordliste.md#kompileringsfeil) |
