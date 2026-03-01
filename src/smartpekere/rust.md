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
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:box_grunnleggende}}
```

Et vanlig bruksområde er rekursive typer, som ellers ville hatt ukjent størrelse:

```rust
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:box_rekursiv}}
```

Uten `Box` kan ikke kompilatoren beregne størrelsen til `Liste`, fordi den inneholder seg selv. `Box` bryter rekursjonen med en peker av fast størrelse.

## `Rc<T>` — referansetelling (enkeltrådet)

Noen ganger trenger flere deler av programmet å eie samme verdi. `Rc<T>` ([referansetelling](../ordliste.md#referansetelling)) tillater dette:

```rust
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:rc_grunnleggende}}
```

`Rc::clone` kopierer ikke verdien — den øker bare referansetelleren. Verdien frigjøres når siste `Rc` går ut av [scope](../ordliste.md#scope):

```rust
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:rc_counting}}
```

> **Merk:** `Rc<T>` er *ikke* trådsikker. Bruk `Arc<T>` for deling mellom tråder.

## `Weak<T>` — bryte sykluser

Akkurat som i C++ kan sykliske referanser føre til [minnelekkasjer](../ordliste.md#minnelekkasje). `Weak<T>` er en svak referanse som ikke hindrer frigjøring:

```rust
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:weak_syklus}}
```

For å bruke verdien fra en `Weak<T>` kaller du `.upgrade()`, som returnerer `Option<Rc<T>>` — `None` hvis verdien er frigjort.

## `Arc<T>` — atomisk referansetelling

`Arc<T>` er den trådsikre varianten av `Rc<T>`. Den bruker atomiske operasjoner for referansetelleren, slik at den trygt kan deles mellom tråder:

```rust
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:arc_grunnleggende}}
```

Rusts typesystem håndhever dette: `Rc<T>` implementerer ikke `Send`-traiten, så kompilatoren nekter å sende den til en annen tråd. Du *må* bruke `Arc<T>` for [trådsikkerhet](../ordliste.md#traadsikkerhet).

## `RefCell<T>` — indre mutabilitet

Normalt sjekker Rust lånereglene ved kompilering: enten én muterbar referanse, eller flere uforanderlige. `RefCell<T>` flytter denne sjekken til kjøretid, noe som muliggjør [indre mutabilitet](../ordliste.md#indre-mutabilitet):

```rust
{{#rustdoc_include ../../rust/src/smartpekere/mod.rs:refcell_grunnleggende}}
```

Hvis du bryter lånereglene ved kjøretid (f.eks. to muterbare lån samtidig), panicker programmet i stedet for å gi [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

`RefCell<T>` kombineres ofte med `Rc<T>` som `Rc<RefCell<T>>` — delt eierskap med mulighet for mutering.

## Når du trenger smartpekere

| Behov | Type | Merknad |
|-------|------|---------|
| Heap-allokering | `Box<T>` | Enkleste valg, enkel eierskap |
| Rekursive typer | `Box<T>` | Bryter rekursjon med fast størrelse |
| Delt eierskap (én tråd) | `Rc<T>` | Referansetelling |
| Delt eierskap (flere tråder) | `Arc<T>` | Atomisk referansetelling |
| Bryte sykluser | `Weak<T>` | Brukes med `Rc`/`Arc` |
| Mutering gjennom delt referanse | `RefCell<T>` | Låneregler ved kjøretid |
| Delt + muterbar (én tråd) | `Rc<RefCell<T>>` | Vanlig kombinasjon |
