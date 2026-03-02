# Mønstermatching i Rust

I forrige kapittel så vi `match` og `if let` brukt på `Option` og `Result`. Her ser vi hvordan [mønstermatching](../ordliste.md#monstermatching) brukes på egendefinerte typer og i flere sammenhenger — fra [destrukturering](../ordliste.md#destrukturering) av tupler til vakter og `let-else`.

## Egendefinerte enumer med data

Rusts `enum` er en [sumtype](../ordliste.md#sumtype) — hver variant kan bære ulike data. `match` lar deg destrukturere variantene og bruke dataen direkte:

```rust
# use std::f64::consts::PI;
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
{{#include ../../rust/src/monstermatching/mod.rs:figur_match}}
```

Typen `Figur` er definert slik:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

Kompilatoren gir en [uttømmende sjekk](../ordliste.md#uttommende-sjekk): legger du til en ny variant i `Figur` uten å oppdatere `match`-uttrykket, får du en [kompileringsfeil](../ordliste.md#kompileringsfeil). Dette er en garanti som C++ sin `switch` og `std::visit` ikke tilbyr.

## Destrukturering

Mønstre i Rust kan destrukturere tupler, structer og nestede verdier — både i `let`-bindinger og i `match`:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:destrukturering_tuppel}}
```

Merk at Rust kan matche *konstante verdier* direkte i mønsteret (`(0, 0)`) og kombinere dem med variabler (`(x, 0)`). Dette gir svært kompakt og lesbar kode for forgreining basert på struktur.

## Vakter i match-armer

Noen ganger trenger du en ekstra betingelse utover selve mønsteret. Da bruker du en *vakt* — et `if`-uttrykk etter mønsteret:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:match_vakter}}
```

Vakter lar deg kombinere strukturell matching med vilkårlige betingelser. I C++ måtte du brukt `if`/`else if`-kjeder eller lagt logikken inne i en `std::visit`-lambda.

## `let-else`

`let-else` er en kortform for «destrukturer eller returner tidlig». Det er spesielt nyttig for å unngå dyp nesting:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:let_else}}
```

Uten `let-else` måtte du brukt en `match` eller `if let` med nesting. `let-else` holder koden flat og lesbar — den «glade stien» fortsetter rett frem, mens feilhåndteringen skjer i `else`-blokken.

## `while let`

`while let` fortsetter en løkke så lenge et mønster matcher:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:while_let}}
```

Dette er idiomatisk Rust for å tømme en samling element for element. Alternativet ville vært en `loop` med en `match` inni — `while let` er mer konsist.

## `matches!`-makroen

Noen ganger trenger du bare å sjekke *om* en verdi matcher et mønster, uten å trekke ut data. Da er `matches!` nyttig:

```rust
# use std::f64::consts::PI;
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
{{#include ../../rust/src/monstermatching/mod.rs:matches_makro}}
```

`matches!` returnerer `true` eller `false` og er spesielt nyttig i kombinasjon med iteratorer og `filter`.

> Se hvordan dette sammenlignes med C++ i [Sammenlikning](./sammenlikning.md).
