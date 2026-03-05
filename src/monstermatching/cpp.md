# Mønstermatching i C++

C++ har ingen innebygd [mønstermatching](../ordliste.md#monstermatching) som et samlet språkkonsept. For forgreining basert på verdier har du `switch` (begrenset til heltall), `if/else if`-kjeder (fleksible, men uten noen kompilatorgarantier), og strukturerte bindinger for enkel [destrukturering](../ordliste.md#destrukturering). Ingen av disse gir [uttømmende sjekk](../ordliste.md#uttommende-sjekk).

## Begrensningene til `switch`

`switch` er C++ sitt eldste verktøy for forgreining basert på verdier — men det fungerer kun med heltallstyper:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:switch_begrensninger}}
```

Begrensningene er betydelige:
- Kun heltallstyper (`int`, `char`, `enum`) — ikke `std::string` eller egendefinerte typer.
- Ingen [destrukturering](../ordliste.md#destrukturering) — du kan ikke trekke ut data fra sammensatte typer.
- Ingen [uttømmende sjekk](../ordliste.md#uttommende-sjekk) — kompilatoren krever ikke at alle verdier er dekket (selv for `enum`-typer er dette kun en advarsel, ikke en feil).
- `break` er påkrevd — uten det «faller» koden gjennom til neste case.

## `if/else if`-kjeder

Når `switch` ikke strekker til, tyr de fleste til `if/else if`-kjeder. Her er et eksempel der vi beregner arealet av ulike figurer basert på en streng:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:if_else_kjede}}
```

Dette fungerer, men har svakheter:
- **Ingen kompilatorhjelp** — legger du til en ny figurtype, gir kompilatoren ingen advarsel om at du mangler en gren.
- **Strenger er feilutsatt** — en skrivefeil i `"sirkel"` gir feil kjøretidsoppførsel, ikke en [kompileringsfeil](../ordliste.md#kompileringsfeil).
- **Ingen destrukturering** — du kan ikke trekke ut `a`, `b` og `c` basert på *hvilken type figur det er*. Alle parametere må sendes inn uansett.

I Rust løses dette med [sumtyper](../ordliste.md#sumtype) og `match` — der hver variant bærer nøyaktig de dataene den trenger, og kompilatoren sjekker at alle varianter er dekket.

## Strukturerte bindinger

C++17 introduserte strukturerte bindinger med `auto [x, y]` for å [destrukturere](../ordliste.md#destrukturering) tupler, par og enkle aggregater:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:strukturerte_bindinger}}
```

Strukturerte bindinger er nyttige, men har klare begrensninger:
- Kun toppnivå — ingen nestede mønstre (`auto [[a, b], c]` er ugyldig).
- Ingen betingelser — du kan ikke kombinere destrukturering med en sjekk.
- Ingen kobling til forgreining — destrukturering og `switch`/`if` er helt separate mekanismer.

> Rust har et integrert system for mønstermatching som dekker alt dette — les videre i [Mønstermatching i Rust](./rust.md).
