# Levetidsannotasjoner i Rust

I [kapitlet om lån og referanser](../../laan_og_referanser/rust.md) så vi hvordan [lånesjekkeren](../../ordliste.md#laanesjekkeren) håndhever [lånereglene](../../ordliste.md#laaneregler) ved kompilering. Her ser vi på tilfeller der kompilatoren trenger *eksplisitt hjelp* til å forstå sammenhenger mellom referansers [levetider](../../ordliste.md#levetid).

## Levetidsannotasjoner

Noen ganger kan ikke kompilatoren automatisk avgjøre hvor lenge en referanse er gyldig. Tenk på en funksjon som returnerer den lengste av to strenger:

```rust,compile_fail
fn lengste(s1: &str, s2: &str) -> &str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}
```

Kompilatoren vet ikke om returverdien følger [levetiden](../../ordliste.md#levetid) til `s1` eller `s2`. Løsningen er en [levetidsannotering](../../ordliste.md#levetidsannotering) — en eksplisitt markering av at referansene henger sammen:

```rust
{{#include ../../../rust/src/levetider/mod.rs:levetid_annotasjon}}
```

Annotasjonen `'a` sier: «returverdien lever minst like lenge som *begge* inputreferansene». Kompilatoren bruker dette til å garantere at resultatet aldri overlever dataen det peker på.

> **Merk:** Levetidsannotasjoner endrer ikke hvor lenge verdier lever — de *beskriver* forholdet mellom referansers levetider slik at kompilatoren kan verifisere dem.

## Levetidselisjon

I mange vanlige tilfeller trenger du ikke skrive levetidsannotasjoner eksplisitt. Rust har tre [elisjonsregler](../../ordliste.md#levetidselisjon) som dekker de fleste funksjoner:

1. Hver inputreferanse får sin egen levetidsparameter.
2. Hvis det er nøyaktig én inputlevetid, brukes den for alle outputreferanser.
3. Hvis en av inputparametrene er `&self` eller `&mut self`, brukes `self`s levetid for alle outputreferanser.

Disse reglene gjør at de fleste funksjoner bare fungerer uten annotasjoner:

```rust
{{#include ../../../rust/src/levetider/mod.rs:levetid_elisjon}}
```

Her utleder kompilatoren automatisk at returverdien har samme levetid som `tekst`-parameteren (regel 2). Du trenger ingen `'a`.

## Levetider i strukturer

Når en struct inneholder en referanse, må du oppgi en levetidsparameter. Dette forteller kompilatoren at strukturen ikke kan overleve dataen den refererer til:

```rust,ignore
{{#include ../../../rust/src/levetider/mod.rs:levetid_struct_type}}
```

Bruk:

```rust
# struct Utdrag<'a> {
#     tekst: &'a str,
# }
# impl<'a> Utdrag<'a> {
#     fn ny(tekst: &'a str) -> Utdrag<'a> {
#         Utdrag { tekst }
#     }
#     fn vis(&self) {
#         println!("  Utdrag: «{}»", self.tekst);
#     }
# }
{{#include ../../../rust/src/levetider/mod.rs:levetid_struct_bruk}}
```

`Utdrag<'a>` kan ikke overleve `roman` — kompilatoren garanterer dette. I C++ ville en tilsvarende struct med en `std::string_view` eller `const char*` stille tillatt at dataen ble destruert mens strukturen fortsatt eksisterte.

## `'static`-levetiden

`'static` er en spesiell levetid som betyr «lever like lenge som hele programmet». Streng-literaler har alltid levetiden `'static`:

```rust
{{#include ../../../rust/src/levetider/mod.rs:levetid_static}}
```

`'static` betyr ikke at verdien er uforanderlig eller global — det betyr bare at den *kan* leve like lenge som programmet. Eide typer som `String` og `i32` oppfyller også `'static`-kravet, fordi de ikke inneholder referanser som kan bli ugyldige.
