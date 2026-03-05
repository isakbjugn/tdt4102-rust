# `std::variant` og `std::visit`

> **Merk:** Denne siden bruker C++-konsepter som ikke er dekket i TDT4102 — lambdaer, templates og `std::variant` (C++17). Den er ment som fordypning for dem som er nysgjerrige på hvordan C++ tilnærmer seg [sumtyper](../ordliste.md#sumtype).

## `std::variant` — type-sikre unioner

I [C++-kapittelet om mønstermatching](../monstermatching/cpp.md) så vi at `if/else if`-kjeder med strenger er feilutsatte. C++ har en bedre mekanisme: `std::variant` fra C++17. En `std::variant` er en [sumtype](../ordliste.md#sumtype) som kan holde én av flere angitte typer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:variant_grunnleggende}}
```

`std::variant` vet alltid hvilken type den inneholder. Det er tryggere enn en vanlig C-`union`, der du selv må holde styr på hvilken type som er aktiv.

Til sammenligning er Rusts `enum` også en sumtype, men med navngitte varianter:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

En viktig forskjell: Rusts varianter har *navn* (`Sirkel`, `Rektangel`), mens `std::variant` sine varianter identifiseres av *type* (`int`, `double`). Det betyr at en `std::variant` ikke kan ha to varianter av *samme type* uten workarounds, mens en Rust-`enum` kan ha f.eks. `Svar(String)` og `Feilmelding(String)` uten problem.

## `std::visit` og Overloaded-mønsteret

For å utføre ulik logikk basert på hvilken type en `std::variant` inneholder, brukes `std::visit` sammen med en *visitor* — et objekt med en `operator()` for hver mulig type.

I praksis kombinerer man gjerne flere lambdaer med en `Overloaded`-template:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:visit_overloaded}}
```

`Overloaded` er en variadisk template som arver fra alle lambdaene og gjør deres `operator()` tilgjengelig. Dette er et etablert mønster i C++17-kode, men det krever forståelse av templates, arv og lambdaer.

Til sammenligning er Rusts `match` innebygget syntaks som gjør det samme:

```rust
# use std::f64::consts::PI;
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
{{#include ../../rust/src/monstermatching/mod.rs:figur_match}}
```

## `std::get` og `std::get_if`

For å hente ut verdien direkte fra en `std::variant` finnes to alternativer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:get_og_get_if}}
```

| Metode | Oppførsel ved feil type |
|--------|------------------------|
| `std::get<T>(v)` | Kaster `std::bad_variant_access` |
| `std::get_if<T>(&v)` | Returnerer `nullptr` |

## Oppsummering

`std::variant` med `std::visit` gir C++ en form for type-sikker dispatch over sumtyper — men syntaksen er tung og krever avanserte språkfunksjoner. Rust sin `match` dekker det samme med enklere syntaks, full [destrukturering](../ordliste.md#destrukturering) og kompilatorgarantert [uttømmende sjekk](../ordliste.md#uttommende-sjekk).
