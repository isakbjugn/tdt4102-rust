# Mønstermatching i Rust

I forrige kapittel så vi `match` og `if let` brukt på `Option` og `Result`. Her ser vi hvordan [mønstermatching](../ordliste.md#monstermatching) brukes på egendefinerte typer og i flere sammenhenger — fra [destrukturering](../ordliste.md#destrukturering) av tupler til sjekker i match-armer og `let-else`.

## Enumer i Rust — mer enn bare konstanter

Fra C++ kjenner du `enum class` som en liste med navngitte konstanter:

```cpp
enum class Farge { Rod, Gronn, Bla };
```

Rusts `enum` kan det samme, men har en avgjørende tilleggsfunksjon: hver variant kan *bære data*. Dette gjør `enum` til en [sumtype](../ordliste.md#sumtype) — en type som kan være én av flere varianter, der hver variant har sin egen datastruktur:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:figur_type}}
```

Her er `Figur` en type med tre varianter: `Sirkel` har én `f64` (radius), `Rektangel` har to (bredde og høyde), og `Trekant` har tre (sidene). Du trenger ikke separate klasser eller en streng-basert `if/else if`-kjede — typen selv beskriver alle mulighetene.

`Option<T>` og `Result<T, E>` fra forrige kapittel er også vanlige enumer definert på akkurat denne måten.

## `match` med destrukturering

`match` lar deg forgrene basert på hvilken variant en enum har, og samtidig trekke ut dataen:

```rust
# use std::f64::consts::PI;
# enum Figur {
#     Sirkel(f64),
#     Rektangel(f64, f64),
#     Trekant(f64, f64, f64),
# }
{{#include ../../rust/src/monstermatching/mod.rs:figur_match}}
```

Kompilatoren gir en [uttømmende sjekk](../ordliste.md#uttommende-sjekk): legger du til en ny variant i `Figur` uten å oppdatere `match`-uttrykket, får du en [kompileringsfeil](../ordliste.md#kompileringsfeil). Dette er en garanti som C++ sin `switch` ikke tilbyr.

## Destrukturering

Mønstre i Rust kan destrukturere tupler, structer og nestede verdier — både i `let`-bindinger og i `match`:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:destrukturering_tuppel}}
```

Merk at Rust kan matche *konstante verdier* direkte i mønsteret (`(0, 0)`) og kombinere dem med variabler (`(x, 0)`). Dette gir svært kompakt og lesbar kode for forgreining basert på struktur.

## Sjekker i match-armer

Noen ganger trenger du en ekstra betingelse utover selve mønsteret. Da bruker du en *sjekk* (eng. *match guard*) — et `if`-uttrykk etter mønsteret:

```rust
{{#include ../../rust/src/monstermatching/mod.rs:match_vakter}}
```

Sjekker lar deg kombinere strukturell matching med vilkårlige betingelser. I C++ måtte du brukt `if`/`else if`-kjeder for å oppnå det samme.

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
