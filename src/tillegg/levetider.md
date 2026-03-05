# Levetidsannotasjoner

> **Merk:** Denne siden forutsetter kjennskap til Rusts [låneregler](../ordliste.md#laaneregler) og [lånesjekkeren](../ordliste.md#laanesjekkeren), som dekkes i [kapitlet om lån og referanser](../laan_og_referanser/README.md). Her ser vi på tilfeller der kompilatoren trenger *eksplisitt hjelp* til å forstå sammenhenger mellom referansers [levetider](../ordliste.md#levetid).

I de fleste tilfeller klarer kompilatoren å utlede levetider automatisk. Men noen ganger — spesielt når en funksjon returnerer en referanse som kan stamme fra flere inputparametere, eller når en struct lagrer en referanse — trenger kompilatoren hjelp. Da bruker vi [levetidsannotasjoner](../ordliste.md#levetidsannotering).

## Når kompilatoren trenger hjelp

Tenk på en funksjon som returnerer den lengste av to strenger:

```rust,compile_fail
fn lengste(s1: &str, s2: &str) -> &str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}
```

Kompilatoren vet ikke om returverdien følger [levetiden](../ordliste.md#levetid) til `s1` eller `s2`. Løsningen er en levetidsannotering — en eksplisitt markering av at referansene henger sammen:

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

## Sammenlikning med C++

C++ har ikke noe konsept som tilsvarer levetidsannotasjoner. Det finnes ingen syntaks for å uttrykke sammenhenger mellom referansers levetider, og kompilatoren gjør ingen slik verifisering.

| Egenskap | C++ | Rust |
|----------|-----|------|
| [Levetider](../ordliste.md#levetid) | Implisitte, ikke verifisert | Sporet av kompilatoren |
| Annotasjonssyntaks | *(finnes ikke)* | `'a`, `'static` |
| [Levetidselisjon](../ordliste.md#levetidselisjon) | *(ikke relevant)* | Tre automatiske regler |
| Referanser i strukturer | Ingen spesiell syntaks | Krever [levetidsannotering](../ordliste.md#levetidsannotering) |

**Levetidsannotasjoner er sjelden nødvendige.** [Elisjonsreglene](../ordliste.md#levetidselisjon) dekker de aller fleste funksjoner automatisk. I praksis trenger du bare eksplisitte annotasjoner når en funksjon returnerer en referanse som kan komme fra flere ulike inputparametere, eller når en struct lagrer en referanse. Dette gjør at levetidssystemet sjelden er i veien i daglig koding — men det er alltid der som sikkerhetsnett.
