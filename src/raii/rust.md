# RAII i Rust

## `Drop`-traiten

Rusts svar på [destruktører](../ordliste.md#destruktor) er [`Drop`](../ordliste.md#drop)-traiten. Når en verdi går ut av [scope](../ordliste.md#scope), kaller Rust automatisk `drop()`-metoden hvis typen implementerer `Drop`. Vi så dette i bruk med [smartpekere](../ordliste.md#smartpeker) i forrige kapittel — her ser vi på det generelle mønsteret.

## Filhåndtering

`std::fs::File` implementerer `Drop`, så filen lukkes automatisk når variabelen går ut av scope:

```rust,no_run
# use std::fs::File;
# use std::io::Write;
{{#include ../../rust/src/raii/mod.rs:raii_fil}}
```

Du trenger aldri å lukke filer manuelt i Rust. Kompilatorens [eierskapsmodell](../ordliste.md#eierskap) garanterer at `Drop` alltid kalles — uansett om funksjonen returnerer normalt eller ved en `panic`.

## Custom `Drop`: `LoggFil`

Du kan implementere `Drop` for egne typer. Her er en loggfilklasse som tilsvarer C++-eksempelet:

```rust,ignore
{{#include ../../rust/src/raii/mod.rs:raii_custom_loggfil_type}}
```

Bruk:

```rust,no_run
# use std::fs::File;
# use std::io::Write;
# struct LoggFil { fil: File, filnavn: String }
# impl LoggFil {
#     fn ny(filnavn: &str) -> LoggFil {
#         let mut fil = File::create(filnavn).unwrap();
#         println!("  Åpner loggfil: {filnavn}");
#         writeln!(fil, "=== Logg startet ===").unwrap();
#         LoggFil { fil, filnavn: filnavn.to_string() }
#     }
#     fn skriv(&mut self, melding: &str) {
#         writeln!(self.fil, "{melding}").unwrap();
#     }
# }
# impl Drop for LoggFil {
#     fn drop(&mut self) {
#         writeln!(self.fil, "=== Logg avsluttet ===").unwrap();
#         println!("  Lukker loggfil: {}", self.filnavn);
#     }
# }
{{#include ../../rust/src/raii/mod.rs:raii_custom_loggfil}}
```

Merk at `LoggFil` eier en `File`, som selv implementerer `Drop`. Rust dropper feltene *etter* at vår `drop()`-metode har kjørt — så vi kan fortsatt skrive til filen i destruktøren.

## `MutexGuard` — RAII for låser

`Mutex::lock()` returnerer en `MutexGuard` — en RAII-vakt som frigjør låsen automatisk når den går ut av scope:

```rust
# use std::sync::Mutex;
{{#include ../../rust/src/raii/mod.rs:raii_mutex}}
```

I motsetning til C++ kan du ikke glemme å låse opp — `MutexGuard` er den *eneste* måten å få tilgang til dataen inni en `Mutex`.

## Drop-rekkefølge

Akkurat som i C++ droppes lokale variabler i *omvendt* deklarasjonsrekkefølge (LIFO):

```rust,ignore
{{#include ../../rust/src/raii/mod.rs:raii_drop_rekkefolge_type}}
```

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

Utskrift:
```text
  Oppretter: A
  Oppretter: B
  Oppretter: C
  Frigjør:   C
  Frigjør:   B
  Frigjør:   A
```

## Eksplisitt tidlig frigjøring med `drop()`

Noen ganger vil du frigjøre en ressurs *før* scopet slutter — for eksempel slippe en lås tidlig. Rust lar deg gjøre dette med `std::mem::drop()`:

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

Utskrift:
```text
  Oppretter: X
  Oppretter: Y
  Før drop
  Frigjør:   X
  Etter drop
  Frigjør:   Y
```

`drop()` tar eierskap over verdien, og kompilatoren nekter deg å bruke den etterpå. Dette er en viktig forskjell fra C++, der du teknisk sett *kan* kalle en destruktør manuelt og deretter fortsette å bruke objektet — noe som gir [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

> **Merk:** Du kan ikke kalle `.drop()` direkte på en verdi — Rust forbyr dette for å unngå [double free](../ordliste.md#double-free). Funksjonen `std::mem::drop()` (eller bare `drop()`) er i stedet en vanlig funksjon som tar eierskap og lar verdien gå ut av scope.
