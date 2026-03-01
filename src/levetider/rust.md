# Levetider i Rust

## Lån og referanser

I Rust kalles det å ta en referanse for å *[låne](../ordliste.md#laan)* en verdi. Du kan låne uforanderlig (`&T`) eller muterbart (`&mut T`). [Lånereglene](../ordliste.md#laaneregler) sier at du på ethvert tidspunkt kan ha *enten* én muterbar referanse *eller* et vilkårlig antall uforanderlige referanser — aldri begge samtidig.

```rust
{{#include ../../rust/src/levetider/mod.rs:levetid_laan_grunnleggende}}
```

Disse reglene håndheves av [lånesjekkeren](../ordliste.md#laanesjekkeren) ved kompilering. Prøver du å bryte dem, får du en [kompileringsfeil](../ordliste.md#kompileringsfeil) — ikke [udefinert oppførsel](../ordliste.md#udefinert-oppforsel) ved kjøretid.

## Lånesjekkeren i aksjon

La oss se på de tre C++-problemene fra forrige side — og hvordan Rust fanger dem ved kompilering.

**Retur av referanse til lokal variabel:**

```rust,compile_fail
fn hent_referanse() -> &String {
    let lokal = String::from("hei");
    &lokal // Kompileringsfeil: lokal lever ikke lenge nok
}
```

Rust nekter å kompilere dette. Referansen ville pekt på en verdi som destrueres når funksjonen returnerer — en [dangling pointer](../ordliste.md#dangling-pointer) som aldri oppstår.

**Referanse inn i avsluttet scope:**

```rust,compile_fail
let r;
{
    let lokal = 42;
    r = &lokal; // Kompileringsfeil: lokal lever ikke lenge nok
}
println!("{r}");
```

Lånesjekkeren ser at `lokal` går ut av [scope](../ordliste.md#scope) før `r` brukes, og nekter å kompilere.

**Vektor-invalidering:**

```rust,compile_fail
let mut tall = vec![1, 2, 3];
let ref_til_forste = &tall[0]; // uforanderlig lån
tall.push(4);                  // muterbart lån — kompileringsfeil!
println!("{ref_til_forste}");
```

`push` krever muterbar tilgang til vektoren (`&mut self`), men det finnes allerede et uforanderlig lån (`ref_til_forste`). Lånereglene forbyr dette, og kompilatoren gir feil.

## Levetidsannotasjoner

Noen ganger kan ikke kompilatoren automatisk avgjøre hvor lenge en referanse er gyldig. Tenk på en funksjon som returnerer den lengste av to strenger:

```rust,compile_fail
fn lengste(s1: &str, s2: &str) -> &str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}
```

Kompilatoren vet ikke om returverdien følger [levetiden](../ordliste.md#levetid) til `s1` eller `s2`. Løsningen er en [levetidsannotering](../ordliste.md#levetidsannotering) — en eksplisitt markering av at referansene henger sammen:

```rust
{{#include ../../rust/src/levetider/mod.rs:levetid_annotasjon}}
```

Annotasjonen `'a` sier: «returverdien lever minst like lenge som *begge* inputreferansene». Kompilatoren bruker dette til å garantere at resultatet aldri overlever dataen det peker på.

> **Merk:** Levetidsannotasjoner endrer ikke hvor lenge verdier lever — de *beskriver* forholdet mellom referansers levetider slik at kompilatoren kan verifisere dem.

## Levetidselisjon

I mange vanlige tilfeller trenger du ikke skrive levetidsannotasjoner eksplisitt. Rust har tre [elisjonsregler](../ordliste.md#levetidselisjon) som dekker de fleste funksjoner:

1. Hver inputreferanse får sin egen levetidsparameter.
2. Hvis det er nøyaktig én inputlevetid, brukes den for alle outputreferanser.
3. Hvis en av inputparametrene er `&self` eller `&mut self`, brukes `self`s levetid for alle outputreferanser.

Disse reglene gjør at de fleste funksjoner bare fungerer uten annotasjoner:

```rust
{{#include ../../rust/src/levetider/mod.rs:levetid_elisjon}}
```

Her utleder kompilatoren automatisk at returverdien har samme levetid som `tekst`-parameteren (regel 2). Du trenger ingen `'a`.

## Levetider i strukturer

Når en struct inneholder en referanse, må du oppgi en levetidsparameter. Dette forteller kompilatoren at strukturen ikke kan overleve dataen den refererer til:

```rust,ignore
{{#include ../../rust/src/levetider/mod.rs:levetid_struct_type}}
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
{{#include ../../rust/src/levetider/mod.rs:levetid_struct_bruk}}
```

`Utdrag<'a>` kan ikke overleve `roman` — kompilatoren garanterer dette. I C++ ville en tilsvarende struct med en `std::string_view` eller `const char*` stille tillatt at dataen ble destruert mens strukturen fortsatt eksisterte.

## `'static`-levetiden

`'static` er en spesiell levetid som betyr «lever like lenge som hele programmet». Streng-literaler har alltid levetiden `'static`:

```rust
{{#include ../../rust/src/levetider/mod.rs:levetid_static}}
```

`'static` betyr ikke at verdien er uforanderlig eller global — det betyr bare at den *kan* leve like lenge som programmet. Eide typer som `String` og `i32` oppfyller også `'static`-kravet, fordi de ikke inneholder referanser som kan bli ugyldige.
