# Minnehåndtering i C++

I C++ har du direkte kontroll over minneallokering og -frigjøring. Dette gir fleksibilitet og ytelse, men også ansvar.

## Heap vs. Stack

- **Stack**: Automatisk allokert, frigjøres når variabelen går ut av [scope](../ordliste.md#scope). Begrenset størrelse.
- **Heap**: Manuelt allokert med `new`, må frigjøres med `delete`. Større og mer fleksibelt.

## `new` og `delete`

```cpp
{{#include ../../cpp/minnehandtering/main.cpp:enkel_allokering}}
```

## `new[]` og `delete[]`

For arrays må du bruke `delete[]`:

```cpp
int* arr = new int[10];
// ... bruk arr ...
delete[] arr;           // NB: delete[], ikke delete
```

Å bruke `delete` på en array (uten `[]`) gir [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

## Vanlige feil

1. **[Minnelekkasje](../ordliste.md#minnelekkasje)** — glemmer `delete`
2. **[Double free](../ordliste.md#double-free)** — kaller `delete` to ganger på samme peker
3. **[Use after free](../ordliste.md#use-after-free)** — bruker pekeren etter `delete`
4. **Feil delete-variant** — `delete` på array eller `delete[]` på enkelt-objekt

## Udefinert oppførsel (Undefined Behavior)

Verken kompilatoren eller clangd advarer om "use after free". Koden kompilerer og kjører, men oppførselen er *udefinert* — C++-standarden garanterer ingenting. I praksis kan du få:

1. **Gammel verdi** — minnet er ikke overskrevet ennå
2. **Søppelverdi** — minnet er gjenbrukt til noe annet
3. **Krasj** (SIGSEGV) — minnet er returnert til OS og utilgjengelig
4. **"Riktig" oppførsel** — ved ren tilfeldighet

Hva som skjer avhenger av:

- **Allokator** — glibc malloc, jemalloc, tcmalloc oppfører seg forskjellig
- **OS** — når/om minnet returneres til systemet
- **Timing** — om noe annet har rukket å allokere det samme minnet
- **Kompilatoroptimalisering** — kompilatoren kan *anta* at UB aldri skjer, og optimalisere bort kode basert på det

Det siste punktet er viktig: kompilatoren har lov til å resonnere "denne koden har UB, men UB skjer aldri i gyldige programmer, ergo denne koden kjøres aldri" — og fjerne den helt.

**Du kan ikke stole på noe etter UB, ikke engang at programmet krasjer forutsigbart.**

Koden som demonstrerer use-after-free:

```cpp
{{#include ../../cpp/minnehandtering/main.cpp:use_after_free}}
```

> Rust fanger disse feilene ved kompilering — les videre i [Minnehåndtering i Rust](./rust.md).
