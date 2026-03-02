# Introduksjon

Et undervisningsprosjekt for å utforske grunnleggende C++-konsepter, med fokus på hvordan disse sammenlikner med moderne språk som Rust.

## Konsepter

Prosjektet tar for seg sentrale konsepter i C++:

- **Null-sikkerhet** - `nullptr` og udefinert oppførsel i C++ vs. Rusts [`Option<T>`](./ordliste.md#option) og [`Result<T, E>`](./ordliste.md#result)
- **Minnehåndtering** - manuell [allokering](./ordliste.md#allokering) med `new`/`delete` vs. Rusts [eierskap-modell](./ordliste.md#eierskap)
- **Smartpekere** - `unique_ptr`, `shared_ptr`, `weak_ptr` vs. Rusts `Box`, `Rc`, `Arc`
- **RAII** - Resource Acquisition Is Initialization, et mønster begge språk bygger på
- **Levetider** - implisitte i C++, eksplisitte i Rust
