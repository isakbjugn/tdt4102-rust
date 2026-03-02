# Mønstermatching i C++

C++ har ingen innebygd mønstermatching som et samlet språkkonsept. I stedet finnes flere mekanismer som til sammen dekker deler av behovet: `switch` for heltall, `std::variant` med `std::visit` for [sumtyper](../ordliste.md#sumtype), og strukturerte bindinger for [destrukturering](../ordliste.md#destrukturering). Hver har sine begrensninger.

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

## `std::variant` — type-sikre unioner

C++17 introduserte `std::variant<T...>` som en type-sikker union — en [sumtype](../ordliste.md#sumtype) som kan holde én av flere angitte typer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:variant_grunnleggende}}
```

`std::variant` vet alltid hvilken type den inneholder, og forhindrer at du leser feil type uten sjekk. Men syntaksen for å jobbe med varianter er vesentlig mer tungvint enn Rusts `match`.

## `std::visit` og Overloaded-mønsteret

For å utføre ulik logikk basert på hvilken type en `std::variant` inneholder, brukes `std::visit` sammen med en *visitor* — et objekt med en `operator()` for hver mulig type:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:visit_overloaded}}
```

`Overloaded`-templaten er et vanlig mønster for å kombinere flere lambdaer til én visitor. Merk at:
- Det er ingen [uttømmende sjekk](../ordliste.md#uttommende-sjekk) — glemmer du en type, får du en kryptisk template-feil, ikke en tydelig melding.
- Syntaksen er tung sammenlignet med Rusts `match`-uttrykk.
- `std::visit` kan returnere en verdi, men alle lambdaer må returnere samme type.

## `std::get` og `std::get_if`

For å hente ut verdien direkte fra en `std::variant` finnes to alternativer:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:get_og_get_if}}
```

| Metode | Oppførsel ved feil type |
|--------|------------------------|
| `std::get<T>(v)` | Kaster `std::bad_variant_access` |
| `std::get_if<T>(&v)` | Returnerer `nullptr` |

`std::get_if` er sikrere, men krever at du jobber med pekere. Ingen av dem gir den kombinerte kraften av destrukturering og uttømmende sjekk som Rusts `match` tilbyr.

## Strukturerte bindinger

C++17 introduserte strukturerte bindinger med `auto [x, y]` for å [destrukturere](../ordliste.md#destrukturering) tupler, par og enkle aggregater:

```cpp
{{#include ../../cpp/monstermatching/main.cpp:strukturerte_bindinger}}
```

Strukturerte bindinger er nyttige, men har klare begrensninger:
- Kun toppnivå — ingen nestede mønstre (`auto [[a, b], c]` er ugyldig).
- Ingen betingelser — du kan ikke kombinere destrukturering med en sjekk.
- Fungerer ikke med `std::variant` — du kan ikke destrukturere en variant direkte.

> Rust har et integrert system for mønstermatching som dekker alt dette — les videre i [Mønstermatching i Rust](./rust.md).
