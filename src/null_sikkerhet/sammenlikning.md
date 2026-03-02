# Sammenlikning

## Oversikt

| C++ | Rust | Merknad |
|-----|------|---------|
| `nullptr` / rå peker | *(finnes ikke)* | Rust har ikke null-konseptet |
| Returner `nullptr` for «ingen verdi» | `Option<T>` | Kompilatoren tvinger håndtering i Rust |
| Returner feilkode / kast unntak | `Result<T, E>` | Feil er verdier, ikke kontrollflyt |
| `std::optional<T>` (C++17) | `Option<T>` | Lignende konsept, men frivillig sjekk i C++ |
| `std::expected<T, E>` (C++23) | `Result<T, E>` | Lignende konsept, men frivillig sjekk i C++ |
| `std::map::find` returnerer iterator | `HashMap::get` returnerer `Option<&V>` | Samme mønster overalt i Rust |

## `std::optional<T>` (C++17)

C++17 introduserte `std::optional<T>` — en type som eksplisitt uttrykker «kanskje en verdi»:

```cpp
{{#include ../../cpp/null_sikkerhet/main.cpp:optional_eksempel}}
```

Dette er konseptuelt likt `Option<T>` i Rust. Forskjellen er at C++ ikke *tvinger* deg til å sjekke:

```cpp
std::optional<int> opt = std::nullopt;
int verdi = opt.value(); // Kaster std::bad_optional_access ved kjøretid
int farlig = *opt;       // UB — ingen unntak, ingen advarsel
```

I Rust ville tilsvarende kode vært en kompileringsfeil — du *må* bruke `match`, `if let`, eller en metode som `unwrap_or` for å hente verdien ut av en `Option`.

## `std::expected<T, E>` (C++23)

C++23 introduserte `std::expected<T, E>` — en type som enten inneholder en verdi eller en feilbeskrivelse:

```cpp
{{#include ../../cpp/null_sikkerhet/main.cpp:expected_eksempel}}
```

Dette er konseptuelt likt `Result<T, E>` i Rust. Men igjen — C++ tvinger deg ikke til å sjekke, og `std::expected` mangler en ekvivalent til `?`-operatoren.

## HashMap vs. std::map

Et konkret eksempel på forskjellen i praksis: oppslag i et kart.

I C++ returnerer `std::map::find` en iterator som kan være `end()` — og det er ditt ansvar å sjekke:

```cpp
{{#include ../../cpp/null_sikkerhet/main.cpp:map_find_null}}
```

I Rust returnerer `HashMap::get` en `Option<&V>` — kompilatoren tvinger deg til å håndtere tilfellet der nøkkelen ikke finnes:

```rust
# use std::collections::HashMap;
{{#include ../../rust/src/null_sikkerhet/mod.rs:hashmap_option}}
```

## Nøkkelforskjellen

C++ og Rust har lignende *typer* (`std::optional` ≈ `Option`, `std::expected` ≈ `Result`), men en fundamental forskjell i *tilnærming*:

| | C++ | Rust |
|-|-----|------|
| **Håndtering** | Frivillig — du *bør* sjekke | Påkrevd — du *må* sjekke |
| **Glemmer du sjekken** | Kompilerer fint, feiler ved kjøretid | [Kompileringsfeil](../ordliste.md#kompileringsfeil) |
| **Feilpropagering** | Manuell / unntak | `?`-operatoren |
| **Konsistens** | Blanding av null, optional, unntak | `Option` og `Result` brukes overalt |

I Rust er [null-sikkerhet](../ordliste.md#null-sikkerhet) ikke en konvensjon du må huske å følge — det er en garanti fra typesystemet.
