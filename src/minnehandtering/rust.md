# Minnehåndtering i Rust

Rust bruker en [eierskapsmodell](../ordliste.md#eierskap) for minnehåndtering: hver verdi har nøyaktig én eier, og minnet frigjøres automatisk når eieren går ut av [scope](../ordliste.md#scope). Dette gjør [minnelekkasjer](../ordliste.md#minnelekkasje) og [use after free](../ordliste.md#use-after-free) til kompileringsfeil i stedet for kjøretidsfeil.

Hvordan verdier overføres mellom variabler og funksjoner avhenger av typen: primitive typer *kopieres*, mens [heap](../ordliste.md#heap)-allokerte typer *flyttes*.

## Copy-semantikk

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

## Move-semantikk

Heap-allokerte typer som `String` blir *flyttet* ved tilordning:

```rust
{{#include ../../rust/src/minnehandtering/mod.rs:move_semantikk}}
```

Det samme gjelder ved funksjonskall:

```rust
# fn function_that_moves(string: String) {
#     println!("Den innsendte variablen har verdien {}", string);
# }
{{#include ../../rust/src/minnehandtering/mod.rs:move_funksjon}}
```

Hvis vi prøver å bruke `c_string` etter at den er flyttet, får vi en [kompileringsfeil](../ordliste.md#kompileringsfeil):

```rust,compile_fail
#   fn function_that_moves(string: String) {
#       println!("Den innsendte variablen har verdien {}", string);
#   }
    let c_string = String::from("Nok en fin streng!");
    function_that_moves(c_string);
    println!("{c_string}"); // Kompileringsfeil! Verdien er flyttet.
```

Rusts kompilator fanger disse feilene *før* programmet kjører — i motsetning til C++, der de gir [udefinert oppførsel](../ordliste.md#udefinert-oppforsel) ved kjøretid.
