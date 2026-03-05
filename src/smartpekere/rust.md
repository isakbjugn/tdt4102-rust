# Smartpekere i Rust

## Eierskap som standardmekanisme

I Rust håndterer [eierskapsmodellen](../ordliste.md#eierskap) det meste av minnehåndteringen. Du trenger ikke [smartpekere](../ordliste.md#smartpeker) for å unngå lekkasjer eller use-after-free — det er allerede garantert av kompilatoren.

Smartpekere i Rust brukes for spesielle scenarier:
- Heap-allokering med kjent størrelse (`Box<T>`)
- Delt eierskap (`Rc<T>`, `Arc<T>`)
- [Indre mutabilitet](../ordliste.md#indre-mutabilitet) (`RefCell<T>`)

## `Box<T>` — heap-allokering med eierskap

`Box<T>` legger en verdi på [heapen](../ordliste.md#heap) med enkel eierskap:

```rust
{{#include ../../rust/src/smartpekere/mod.rs:box_grunnleggende}}
```

Et vanlig bruksområde er rekursive typer, som ellers ville hatt ukjent størrelse:

```rust
{{#include ../../rust/src/smartpekere/mod.rs:box_rekursiv_type}}
```

```rust
# enum Liste {
#     Element(i32, Box<Liste>),
#     Slutt,
# }
{{#include ../../rust/src/smartpekere/mod.rs:box_rekursiv}}
```

Uten `Box` kan ikke kompilatoren beregne størrelsen til `Liste`, fordi den inneholder seg selv. `Box` bryter rekursjonen med en peker av fast størrelse.

## `Rc<T>` — referansetelling (én-trådet)

Noen ganger trenger flere deler av programmet å eie samme verdi. `Rc<T>` ([referansetelling](../ordliste.md#referansetelling)) tillater dette:

```rust
# use std::rc::Rc;
{{#include ../../rust/src/smartpekere/mod.rs:rc_grunnleggende}}
```

`Rc::clone` kopierer ikke verdien — den øker bare referansetelleren. Verdien frigjøres når siste `Rc` går ut av [scope](../ordliste.md#scope):

```rust
# use std::rc::Rc;
{{#include ../../rust/src/smartpekere/mod.rs:rc_counting}}
```

> **Merk:** `Rc<T>` er *ikke* trådsikker. For deling mellom tråder finnes `Arc<T>` — se [tillegget om trådsikkerhet](../tillegg/traader.md).

## `Weak<T>` — bryte sykluser

Akkurat som i C++ kan sykliske referanser føre til [minnelekkasjer](../ordliste.md#minnelekkasje). `Weak<T>` er en svak referanse som ikke hindrer frigjøring:

```rust,ignore
{{#include ../../rust/src/smartpekere/mod.rs:weak_syklus_type}}
```

```rust
# use std::rc::{Rc, Weak};
# #[derive(Debug)]
# #[allow(dead_code)]
# struct Node {
#     navn: String,
#     neste: Option<Rc<Node>>,
#     forrige: Option<Weak<Node>>,
# }
# impl Drop for Node {
#     fn drop(&mut self) {
#         println!("  ~Node({})", self.navn);
#     }
# }
{{#include ../../rust/src/smartpekere/mod.rs:weak_syklus}}
```

For å bruke verdien fra en `Weak<T>` kaller du `.upgrade()`, som returnerer `Option<Rc<T>>` — `None` hvis verdien er frigjort.

## Når du trenger smartpekere

| Behov | Type | Merknad |
|-------|------|---------|
| Heap-allokering | `Box<T>` | Enkleste valg, enkel eierskap |
| Rekursive typer | `Box<T>` | Bryter rekursjon med fast størrelse |
| Delt eierskap (én tråd) | `Rc<T>` | Referansetelling |
| Bryte sykluser | `Weak<T>` | Brukes med `Rc`/`Arc` |
| Delt eierskap (flere tråder) | `Arc<T>`¹ | Atomisk referansetelling |
| Mutering gjennom delt referanse | `RefCell<T>`² | Låneregler ved kjøretid |

*¹ `Arc<T>` krever forståelse av tråder — se [tillegget om trådsikkerhet](../tillegg/traader.md).*
*² `RefCell<T>` krever forståelse av låneregler — se [tillegget om `RefCell<T>`](../tillegg/refcell.md).*
