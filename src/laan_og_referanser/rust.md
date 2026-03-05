# Lån og referanser i Rust

## Lån og referanser

I Rust kalles det å ta en referanse for å *[låne](../ordliste.md#laan)* en verdi. Du kan låne uforanderlig (`&T`) eller muterbart (`&mut T`). [Lånereglene](../ordliste.md#laaneregler) sier at du på ethvert tidspunkt kan ha *enten* én muterbar referanse *eller* et vilkårlig antall uforanderlige referanser — aldri begge samtidig.

```rust
{{#include ../../rust/src/laan_og_referanser/mod.rs:laan_grunnleggende}}
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

## Videre lesning

Når kompilatoren ikke automatisk kan avgjøre hvor lenge en referanse er gyldig, trenger du *levetidsannotasjoner*. Dette er et mer avansert tema som dekkes i [tillegget om levetider](../tillegg/levetider/README.md).
