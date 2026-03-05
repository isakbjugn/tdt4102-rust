# RAII

> **Merk:** Denne siden forutsetter kjennskap til [destruktører](../ordliste.md#destruktor) og [smartpekere](../ordliste.md#smartpeker), som dekkes i de respektive kapitlene. Her ser vi på RAII som overordnet mønster og sammenlikner mekanismene i C++ og Rust.

## Hva er RAII?

[RAII](../ordliste.md#raii) (*Resource Acquisition Is Initialization*) er et mønster der ressurser knyttes til et objekts levetid: konstruktøren erverver ressursen, og [destruktøren](../ordliste.md#destruktor) frigjør den automatisk når objektet går ut av [scope](../ordliste.md#scope).

Du har allerede brukt RAII i C++ — `std::ofstream` lukker filen i destruktøren, og [smartpekere](../ordliste.md#smartpeker) som `unique_ptr` frigjør minnet:

```cpp
{{#include ../../cpp/raii/main.cpp:raii_fil_med_raii}}
```

Mønsteret gjelder langt mer enn filer og minne: låser, nettverkstilkoblinger og databasetransaksjoner kan alle følge samme prinsipp.

## Destruktørrekkefølge

Lokale variabler destrueres i *omvendt* deklarasjonsrekkefølge (LIFO — sist inn, først ut). Dette gjelder i både C++ og Rust:

```cpp
{{#include ../../cpp/raii/main.cpp:raii_destruktor_rekkefolge}}
```

Utskrift:
```text
  Oppretter: A
  Oppretter: B
  Oppretter: C
  Frigjør:   C
  Frigjør:   B
  Frigjør:   A
```

Dette sikrer at avhengigheter mellom variabler alltid er gyldige under opprydding — senere variabler kan avhenge av tidligere, og de destrueres først.

## Rust: `Drop`-traiten

Rusts svar på destruktører er [`Drop`](../ordliste.md#drop)-traiten. Når en verdi går ut av [scope](../ordliste.md#scope), kaller Rust automatisk `drop()`-metoden:

```rust,ignore
{{#include ../../rust/src/raii/mod.rs:raii_drop_rekkefolge_type}}
```

Rekkefølgen er identisk med C++:

```rust
# struct Ressurs { navn: String }
# impl Ressurs {
#     fn ny(navn: &str) -> Ressurs {
#         println!("  Oppretter: {navn}");
#         Ressurs { navn: navn.to_string() }
#     }
# }
# impl Drop for Ressurs {
#     fn drop(&mut self) {
#         println!("  Frigjør:   {}", self.navn);
#     }
# }
{{#include ../../rust/src/raii/mod.rs:raii_drop_rekkefolge}}
```

## Eksplisitt tidlig frigjøring

Noen ganger vil du frigjøre en ressurs *før* scopet slutter. I Rust bruker du `std::mem::drop()`:

```rust
# struct Ressurs { navn: String }
# impl Ressurs {
#     fn ny(navn: &str) -> Ressurs {
#         println!("  Oppretter: {navn}");
#         Ressurs { navn: navn.to_string() }
#     }
# }
# impl Drop for Ressurs {
#     fn drop(&mut self) {
#         println!("  Frigjør:   {}", self.navn);
#     }
# }
{{#include ../../rust/src/raii/mod.rs:raii_tidlig_drop}}
```

`drop()` tar [eierskap](../ordliste.md#eierskap) over verdien, og kompilatoren nekter deg å bruke den etterpå — en [kompileringsfeil](../ordliste.md#kompileringsfeil). I C++ kan du teknisk sett kalle en destruktør manuelt (`obj.~Klasse()`) og deretter fortsette å bruke objektet — noe som gir [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

> **Merk:** Du kan ikke kalle `.drop()` direkte på en verdi — Rust forbyr dette for å unngå [double free](../ordliste.md#double-free). Funksjonen `std::mem::drop()` er i stedet en vanlig funksjon som tar eierskap og lar verdien gå ut av scope.

## Sammenlikning med C++

| Egenskap | C++ | Rust |
|----------|-----|------|
| RAII-mekanisme | [Destruktør](../ordliste.md#destruktor) (`~Klasse()`) | [`Drop`](../ordliste.md#drop)-trait |
| Manuell tidlig frigjøring | Mulig (men fragilt) | `std::mem::drop(verdi)` |
| Kalle destruktør direkte | Mulig (`obj.~Klasse()`) | Ikke tillatt |
| Rekkefølge (lokale variabler) | Omvendt (LIFO) | Omvendt (LIFO) |
| Rekkefølge (felter) | Deklarasjonsrekkefølge | Deklarasjonsrekkefølge |

**RAII er et mønster du allerede bruker.** I C++ praktiserer du det gjennom `std::ofstream`, `std::lock_guard` og smartpekere — destruktøren rydder opp automatisk. Rust formaliserer det samme gjennom `Drop`-traiten, med den ekstra garantien at [eierskapsmodellen](../ordliste.md#eierskap) gjør det umulig å bruke en ressurs etter at den er frigjort.
