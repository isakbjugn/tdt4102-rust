# Null-sikkerhet i Rust

Rust har ikke `null`. I stedet bruker Rust typesystemet for å uttrykke at en verdi kan mangle — og kompilatoren tvinger deg til å håndtere det. De to viktigste typene er `Option<T>` (verdi som kan mangle) og `Result<T, E>` (operasjon som kan feile).

## `Option<T>` — verdi eller ingenting

[`Option<T>`](../ordliste.md#option) har to varianter: `Some(T)` (en verdi) og `None` (ingen verdi). Du kan ikke bruke verdien uten å først sjekke hvilken variant det er:

```rust
{{#include ../../rust/src/null_sikkerhet/mod.rs:option_grunnleggende}}
```

## Pattern matching

`match` tvinger deg til å håndtere *begge* tilfeller. `if let` er en kortform når du bare bryr deg om ett:

```rust
{{#include ../../rust/src/null_sikkerhet/mod.rs:option_pattern_matching}}
```

Prøver du å bruke en `Option<i32>` som om den var en `i32`, får du en [kompileringsfeil](../ordliste.md#kompileringsfeil):

```rust,compile_fail
let tall: Option<i32> = Some(42);
println!("{}", tall + 1); // Kompileringsfeil! Option<i32> er ikke i32.
```

## Nyttige metoder

`Option` har mange metoder som gjør koden kortere og mer lesbar:

```rust
{{#include ../../rust/src/null_sikkerhet/mod.rs:option_metoder}}
```

| Metode | Beskrivelse |
|--------|-------------|
| `unwrap_or(standard)` | Gir verdien, eller `standard` hvis `None` |
| `map(f)` | Transformerer verdien inni `Some`, lar `None` passere |
| `and_then(f)` | Som `map`, men `f` returnerer selv en `Option` |

## `Result<T, E>` — verdi eller feil

[`Result<T, E>`](../ordliste.md#result) ligner `Option`, men bærer med seg en feilmelding i stedet for bare `None`:

```rust
{{#include ../../rust/src/null_sikkerhet/mod.rs:result_grunnleggende}}
```

## `?`-operatoren

`?`-operatoren gjør feilhåndtering kort og idiomatisk. Den returnerer verdien ved `Ok`, eller propagerer feilen videre ved `Err`:

```rust
# fn result_sporsmalstegn() -> Result<(), String> {
{{#include ../../rust/src/null_sikkerhet/mod.rs:result_sporsmalstegn}}
# Ok(())
# }
```

Uten `?` måtte vi skrevet en `match` for hvert steg — `?`-operatoren fjerner denne boilerplate-koden og gjør det like enkelt å håndtere feil som å ignorere dem.

> Se hvordan dette sammenlignes med C++ sine `std::optional` og `std::expected` i [Sammenlikning](./sammenlikning.md).
