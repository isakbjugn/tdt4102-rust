# Sammenlikning

## Oversikt

| Egenskap | C++ | Rust |
|----------|-----|------|
| [Sumtyper](../ordliste.md#sumtype) | `std::variant<T...>` (C++17)¹ | `enum` |
| Dispatch | `std::visit()` + lambdaer¹ | `match`-uttrykk |
| [Uttømmende sjekk](../ordliste.md#uttommende-sjekk) | Ikke håndhevet | Kompilatorgaranti |
| [Destrukturering](../ordliste.md#destrukturering) | `auto [x, y]` (begrenset) | Full støtte i alle mønstre |
| Vakter | Nei (må bruke `if` i lambda) | Ja (`if`-vakter i match-armer) |
| Som uttrykk | `std::visit` returnerer verdi¹ | `match` er et uttrykk |
| `switch` | Kun heltallstyper | `match` fungerer på alle typer |

*¹ Ikke dekket i TDT4102 — se [tillegget om `std::variant` og `std::visit`](../tillegg/variant_og_visit.md) for detaljer.*

## Sumtyper: `std::variant` vs. `enum`

I C++-kapittelet så vi at `if/else if`-kjeder med strenger er feilutsatte: ingen kompilatorhjelp, ingen destrukturering, og skrivefeil gir feil kjøretidsoppførsel. Grunnproblemet er at C++ mangler en god måte å representere «én av flere varianter med ulike data».

C++17 introduserte faktisk en [sumtype](../ordliste.md#sumtype) for dette: `std::variant`. Den kan holde én av flere angitte typer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:variant_grunnleggende}}
```

Rust har `enum` — også en sumtype, men som et språkkonsept med navngitte varianter:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

Nøkkelforskjellen er at Rusts varianter har *navn* (`Sirkel`, `Rektangel`), mens `std::variant` sine varianter identifiseres av *type* (`int`, `double`). Det betyr at en `std::variant` ikke kan ha to varianter av samme type uten workarounds, mens en Rust-`enum` kan ha `Svar(String)` og `Feilmelding(String)` uten problem.

Rusts `enum` har dessuten disse fordelene over C++-tilnærmingen:

- **Kompilatoren garanterer** at du bare bruker data som varianten faktisk har.
- **Skrivefeil er umulige** — `Figur::Sirkle` gir en [kompileringsfeil](../ordliste.md#kompileringsfeil), mens `"sirkle"` i en streng ikke gjør det.
- **Hver variant bærer nøyaktig sine data** — ingen ubrukte parametere.

> For å utføre dispatch på en `std::variant` bruker C++ `std::visit` med lambdaer og en `Overloaded`-template — avanserte konsepter som ikke er dekket i TDT4102. Se [tillegget om `std::variant` og `std::visit`](../tillegg/variant_og_visit.md) for detaljer.

## Uttømmende sjekk

Rusts `match` krever at du håndterer *alle* varianter:

```rust,compile_fail
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
# let figur = Figur::Sirkel(1.0);
match figur {
    Figur::Sirkel(r) => println!("Sirkel med radius {r}"),
    Figur::Rektangel(b, h) => println!("Rektangel {b}x{h}"),
    // Kompileringsfeil! Trekant er ikke dekket.
}
```

Legger du til en ny variant i `enum`-en, vil kompilatoren peke ut *alle* steder i koden som mangler den nye varianten. I C++ gir `switch` på `enum`-verdier kun en advarsel (ikke en feil), og `if/else if`-kjeder gir ingenting.

## Rask variant-sjekk: `holds_alternative` vs. `matches!`

Legg merke til at `std::variant`-eksempelet over bruker `std::holds_alternative` for å sjekke hvilken type varianten inneholder. Rust har en tilsvarende snarvei — `matches!`-makroen:

| C++ | Rust |
|-----|------|
| `std::holds_alternative<std::string>(verdi)` | `matches!(figur, Figur::Sirkel(_))` |

Begge returnerer `true`/`false`. Forskjellen er at Rusts `matches!` også støtter destrukturering og vakter:

```rust
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
# let figur = Figur::Sirkel(5.0);
// Sjekk variant + betingelse i ett uttrykk
let er_stor_sirkel = matches!(figur, Figur::Sirkel(r) if r > 10.0);
```

I C++ måtte du kombinert `std::holds_alternative` med `std::get` og en separat `if`-sjekk for å oppnå det samme.

## Destrukturering

**Rust** støtter [destrukturering](../ordliste.md#destrukturering) på alle nivåer — tupler, structer, enumer, nestede mønstre, og kombinasjoner:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:destrukturering_tuppel}}
```

**C++** har strukturerte bindinger fra C++17, men kun på toppnivå:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:strukturerte_bindinger}}
```

| | C++ | Rust |
|-|-----|------|
| **Tupler** | `auto [x, y]` | `let (x, y)` |
| **Nestede mønstre** | Nei | `(0, (x, _))` |
| **I match-armer** | Nei | Ja, med verdier og vakter |
| **I løkker** | `auto& [k, v]` i range-for | `(k, v)` i `for`-løkker |

## Nøkkelforskjellen

C++ sine mekanismer er spredt over separate verktøy — `switch`, `if/else if`, strukturerte bindinger — som hver har sine begrensninger og ikke samarbeider. Verktøyene studentene kjenner fra pensum gir ingen [uttømmende sjekk](../ordliste.md#uttommende-sjekk) og begrenset [destrukturering](../ordliste.md#destrukturering).

Rust har *ett* integrert system for [mønstermatching](../ordliste.md#monstermatching) som brukes overalt: i `match`, `if let`, `while let`, `let-else`, `for`-løkker og funksjonsparametere. Alle mønstre støtter destrukturering, og `match` gir alltid uttømmende sjekk. Resultatet er kode som er mer kompakt, mer lesbar og tryggere.
