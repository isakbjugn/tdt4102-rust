# Trådsikkerhet

> **Merk:** Denne siden bruker konsepter fra flertrådsprogrammering (tråder, mutexer, låser) som ikke er dekket i TDT4102. Den er ment som fordypning for dem som er nysgjerrige på hvordan C++ og Rust håndterer [trådsikkerhet](../ordliste.md#traadsikkerhet) med [smartpekere](../ordliste.md#smartpeker) og [RAII](../ordliste.md#raii).

## Kontekst

I [smartpekere-kapittelet](../smartpekere/rust.md) så vi `Rc<T>` for delt eierskap. `Rc<T>` bruker en vanlig (ikke-atomisk) teller, og er derfor *ikke* trådsikker. For deling mellom tråder trenger vi `Arc<T>`.

I [tillegget om RAII](./raii.md) ser vi at RAII-mønsteret brukes til å automatisk frigjøre ressurser. Et klassisk bruksområde er låser (mutexer), der RAII sikrer at låsen alltid slippes — selv ved tidlig retur.

## `Arc<T>` — atomisk referansetelling (Rust)

`Arc<T>` er den trådsikre varianten av `Rc<T>`. Den bruker atomiske operasjoner for [referansetelleren](../ordliste.md#referansetelling), slik at den trygt kan deles mellom tråder:

```rust
# use std::sync::Arc;
# use std::thread;
{{#include ../../rust/src/smartpekere/mod.rs:arc_grunnleggende}}
```

Rusts typesystem håndhever dette: `Rc<T>` implementerer ikke `Send`-traiten, så kompilatoren nekter å sende den til en annen tråd. Du *må* bruke `Arc<T>` for trådsikkerhet.

## `lock_guard` — RAII for låser (C++)

Mutex-låsing er et klassisk RAII-bruksområde. `std::lock_guard` tar låsen i konstruktøren og slipper den i [destruktøren](../ordliste.md#destruktor):

```cpp
{{#include ../../cpp/raii/main.cpp:raii_lock_guard}}
```

Uten `lock_guard` måtte du husket å kalle `mtx.unlock()` — og en glemt `unlock` kan føre til deadlock.

## `MutexGuard` — RAII for låser (Rust)

`Mutex::lock()` returnerer en `MutexGuard` — en RAII-vakt som frigjør låsen automatisk når den går ut av [scope](../ordliste.md#scope):

```rust
# use std::sync::Mutex;
{{#include ../../rust/src/raii/mod.rs:raii_mutex}}
```

I motsetning til C++ kan du ikke glemme å låse opp — `MutexGuard` er den *eneste* måten å få tilgang til dataen inni en `Mutex`.

## Sammenlikning

| Egenskap | C++ | Rust |
|----------|-----|------|
| Trådsikker delt eierskap | `shared_ptr<T>` (kontrollblokk er trådsikker) | `Arc<T>` |
| Ikke-trådsikker delt eierskap | *(finnes ikke som distinkt type)* | `Rc<T>` |
| RAII-lås | `std::lock_guard` | `MutexGuard` |
| Tvungen bruk av RAII for lås | Nei — manuell lock/unlock er mulig | Ja — `MutexGuard` er eneste tilgang |
| Kompilatorhåndhevet trådsikkerhet | Nei | Ja — via `Send` og `Sync` traits |

I C++ har `shared_ptr` trådsikker referansetelling, men skiller ikke mellom én-trådet og fler-trådet bruk. I Rust tvinges du til å velge: `Rc<T>` for én tråd (billigere), `Arc<T>` for flere tråder (trådsikker). Kompilatoren nekter deg å bruke feil variant.
