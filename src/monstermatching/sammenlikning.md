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

## Sumtyper: `enum` som datastruktur

I C++-kapittelet så vi at `if/else if`-kjeder med strenger er feilutsatte: ingen kompilatorhjelp, ingen destrukturering, og skrivefeil gir feil kjøretidsoppførsel. Grunnproblemet er at C++ ikke har noen god måte å representere «én av flere varianter med ulike data» i pensum.

Rust har `enum` — en [sumtype](../ordliste.md#sumtype) der hver variant kan bære ulike data:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

Typen `Figur` uttrykker *direkte* i typesystemet at en figur er enten en sirkel (med radius), et rektangel (med bredde og høyde), eller en trekant (med tre sider). Sammenlignet med C++-versjonen der figurtypen var en streng:

- **Kompilatoren garanterer** at du bare bruker data som varianten faktisk har.
- **Skrivefeil er umulige** — `Figur::Sirkle` gir en [kompileringsfeil](../ordliste.md#kompileringsfeil), mens `"sirkle"` i en streng ikke gjør det.
- **Hver variant bærer nøyaktig sine data** — ingen ubrukte parametere.

> C++ har `std::variant` (C++17) som også er en sumtype, men den er ikke del av TDT4102-pensum. Se [tillegget](../tillegg/variant_og_visit.md) for en sammenligning.

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
