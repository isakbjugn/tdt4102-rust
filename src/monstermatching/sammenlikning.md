# Sammenlikning

## Oversikt

| Egenskap | C++ | Rust |
|----------|-----|------|
| [Sumtyper](../ordliste.md#sumtype) | `std::variant<T...>` (C++17) | `enum` |
| Dispatch | `std::visit()` + lambdaer | `match`-uttrykk |
| [Uttømmende sjekk](../ordliste.md#uttommende-sjekk) | Ikke håndhevet | Kompilatorgaranti |
| [Destrukturering](../ordliste.md#destrukturering) | `auto [x, y]` (begrenset) | Full støtte i alle mønstre |
| Vakter | Nei (må bruke `if` i lambda) | Ja (`if`-vakter i match-armer) |
| Som uttrykk | `std::visit` returnerer verdi | `match` er et uttrykk |
| `switch` | Kun heltallstyper | `match` fungerer på alle typer |

## Sumtyper: `std::variant` vs. `enum`

I C++-kapittelet så vi at `if/else if`-kjeder med strenger er feilutsatte og gir ingen kompilatorhjelp. C++ har faktisk en bedre mekanisme for dette: `std::variant` fra C++17. En `std::variant` er en [sumtype](../ordliste.md#sumtype) — en type som kan holde én av flere angitte typer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:variant_grunnleggende}}
```

Rusts `enum` løser det samme, men som et språkkonsept med navngitte varianter:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

Nøkkelforskjellen er at Rusts varianter har *navn* (`Sirkel`, `Rektangel`), mens C++ sine varianter identifiseres av *type* (`int`, `double`). Dette betyr at en `std::variant` ikke kan ha to varianter av samme type uten workarounds, mens en Rust-`enum` kan ha `Svar(String)` og `Feilmelding(String)` uten problem.

## Dispatch: `std::visit` vs. `match`

For å utføre ulik logikk basert på varianten bruker C++ `std::visit` med lambdaer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:visit_overloaded}}
```

I Rust er dette et `match`-uttrykk:

```rust
# use std::f64::consts::PI;
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
{{#include ../../rust/src/monstermatching/mod.rs:figur_match}}
```

`std::visit` krever en `Overloaded`-template og lambdaer — tungt å lese og skrive. `match` er innebygget syntaks der du destrukturerer variantene direkte. Glemmer du en variant i Rust, får du en [kompileringsfeil](../ordliste.md#kompileringsfeil). Glemmer du en type i `std::visit`, får du en kryptisk template-feil.

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

Legger du til en ny variant i `enum`-en, vil kompilatoren peke ut *alle* steder i koden som mangler den nye varianten. C++ sin `switch` på `enum`-verdier gir kun en advarsel, og `if/else if`-kjeder gir ingenting.

## Destrukturering

**Rust** støtter [destrukturering](../ordliste.md#destrukturering) på alle nivåer — tupler, structer, enumer, nestede mønstre, og kombinasjoner:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:destrukturering_tuppel}}
```

**C++** har strukturerte bindinger, men kun på toppnivå:

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

C++ sine mekanismer for [mønstermatching](../ordliste.md#monstermatching) er spredt over flere separate funksjoner — `switch`, `if/else if`, `std::variant`, `std::visit`, strukturerte bindinger — som hver har sine begrensninger og ikke samarbeider sømløst.

Rust har *ett* integrert system for mønstermatching som brukes overalt: i `match`, `if let`, `while let`, `let-else`, `for`-løkker og funksjonsparametere. Alle mønstre støtter destrukturering, og `match` gir alltid [uttømmende sjekk](../ordliste.md#uttommende-sjekk). Resultatet er kode som er mer kompakt, mer lesbar og tryggere.
