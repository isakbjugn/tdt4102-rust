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

Begge språkene lar deg definere typer med flere varianter, men syntaksen er svært forskjellig.

**C++** — `std::variant` er et bibliotekskonsept med template-parametere:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:variant_grunnleggende}}
```

**Rust** — `enum` er et språkkonsept med navngitte varianter:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

Nøkkelforskjellen er at Rusts varianter har *navn* (`Sirkel`, `Rektangel`), mens C++ sine varianter identifiseres av *type* (`int`, `double`). Dette betyr at en `std::variant` ikke kan ha to varianter av samme type uten workarounds, mens en Rust-`enum` kan ha `Svar(String)` og `Feilmelding(String)` uten problem.

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

Legger du til en ny variant i `enum`-en, vil kompilatoren peke ut *alle* steder i koden som mangler den nye varianten. I C++ gir `std::visit` en template-feil hvis du mangler en type i visitoren, men meldingen er ofte vanskelig å tyde — og `switch` på `enum`-verdier gir kun en advarsel, ikke en feil.

## Destrukturering

**Rust** støtter destrukturering på alle nivåer — tupler, structer, enumer, nestede mønstre, og kombinasjoner:

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

C++ sine mekanismer for [mønstermatching](../ordliste.md#monstermatching) er spredt over flere separate funksjoner — `switch`, `std::variant`, `std::visit`, strukturerte bindinger — som hver har sine begrensninger og ikke samarbeider sømløst.

Rust har *ett* integrert system for mønstermatching som brukes overalt: i `match`, `if let`, `while let`, `let-else`, `for`-løkker og funksjonsparametere. Alle mønstre støtter destrukturering, og `match` gir alltid [uttømmende sjekk](../ordliste.md#uttommende-sjekk). Resultatet er kode som er mer kompakt, mer lesbar og tryggere.
