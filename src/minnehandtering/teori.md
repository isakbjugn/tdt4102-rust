# Minnehåndtering i C++

I C++ har du direkte kontroll over minneallokering og -frigjøring. Dette gir fleksibilitet og ytelse, men også ansvar.

## Heap vs. Stack

- **Stack**: Automatisk allokert, frigjøres når variabelen går ut av scope. Begrenset størrelse.
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

Å bruke `delete` på en array (uten `[]`) gir udefinert oppførsel.

## Vanlige feil

1. **Minnelekkasje** - glemmer `delete`
2. **Double free** - kaller `delete` to ganger på samme peker
3. **Use after free** - bruker pekeren etter `delete`
4. **Feil delete-variant** - `delete` på array eller `delete[]` på enkelt-objekt

## Udefinert oppførsel (Undefined Behavior)

Verken kompilatoren eller clangd advarer om "use after free". Koden kompilerer og kjører, men oppførselen er *udefinert* - C++-standarden garanterer ingenting. I praksis kan du få:

1. **Gammel verdi** - minnet er ikke overskrevet ennå
2. **Søppelverdi** - minnet er gjenbrukt til noe annet
3. **Krasj** (SIGSEGV) - minnet er returnert til OS og utilgjengelig
4. **"Riktig" oppførsel** - ved ren tilfeldighet

Hva som skjer avhenger av:

- **Allokator** - glibc malloc, jemalloc, tcmalloc oppfører seg forskjellig
- **OS** - når/om minnet returneres til systemet
- **Timing** - om noe annet har rukket å allokere det samme minnet
- **Kompilatoroptimalisering** - kompilatoren kan *anta* at UB aldri skjer, og optimalisere bort kode basert på det

Det siste punktet er viktig: kompilatoren har lov til å resonnere "denne koden har UB, men UB skjer aldri i gyldige programmer, ergo denne koden kjøres aldri" - og fjerne den helt.

**Du kan ikke stole på noe etter UB, ikke engang at programmet krasjer forutsigbart.**

Koden som demonstrerer use-after-free:

```cpp
{{#include ../../cpp/minnehandtering/main.cpp:use_after_free}}
```

## Address Sanitizer

For å fange minnefeil i C++ kan du bruke Address Sanitizer (ASan):

```bash
clang++ -std=c++20 -fsanitize=address -g cpp/minnehandtering/main.cpp -o main && ./main
```

ASan gir tydelige feilmeldinger for use-after-free, buffer overflow, og andre minnefeil - men kun ved kjøretid, ikke kompilering.

## Sammenlikning med Rust

| C++ | Rust |
|-----|------|
| `new int(42)` | `Box::new(42)` |
| `delete p` | Automatisk ved scope-slutt |
| Minnelekkasje mulig | Umulig (uten `unsafe`) |
| Use after free mulig | Kompileringsfeil |

### Copy-semantikk i Rust

Primitive typer som `i32` implementerer `Copy` og blir automatisk kopiert:

```rust
{{#include ../../rust/src/minnehandtering/mod.rs:copy_semantikk}}
```

Kopiering fungerer også ved funksjonskall:

```rust
# fn function_that_copies(value: i32) {
#     println!("Den innsendte variablen har verdien {}", value);
# }
{{#include ../../rust/src/minnehandtering/mod.rs:copy_funksjon}}
```

### Move-semantikk i Rust

Heap-allokerte typer som `String` blir *flyttet* ved tilordning:

```rust
{{#include ../../rust/src/minnehandtering/mod.rs:move_semantikk}}
```

Det samme gjelder ved funksjonskall:

```rust,editable
{{#include ../../rust/src/minnehandtering/examples/move_funksjon.rs}}
```

Rusts kompilator fanger disse feilene *før* programmet kjører - i motsetning til C++, der de gir udefinert oppførsel ved kjøretid.
