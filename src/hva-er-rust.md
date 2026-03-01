# Hva er Rust?

Rust er et systemprogrammeringsspråk som ble startet av Graydon Hoare hos Mozilla i 2010 og nådde versjon 1.0 i 2015. Det er designet for å gi deg samme ytelse som C og C++, men med kompilatorgarantier for minnesikkerhet — uten å bruke en [søppelsamler](ordliste.md#soppelsamler).

Rust er multiparadigme: du kan skrive imperativ kode med mutabel tilstand, funksjonell kode med iteratorer og closures, og generisk kode med traits og typeparametere. Syntaksen minner om C++, mens typesystemet er inspirert av Haskell og ML.

For dere som allerede kan C++ er Rust spesielt interessant fordi det løser mange av de samme problemene — men med andre virkemidler. Der C++ gir deg verktøy og konvensjoner for å unngå minnefeil, gjør Rust det til en *kompileringsfeil* å bruke minne feil.

La oss plassere Rust i landskapet av programmeringsspråk med noen tabeller.

## Minnehåndtering

|  | **Uten søppelsamler** | **Med søppelsamler** |
|---|---|---|
| **Minnesikker** | **Rust** | Java, Go, C#, Python, Haskell |
| **Ikke minnesikker** | C, C++ | |

Rust er alene i øvre venstre kvadrant — det eneste utbredte språket som er minnesikkert uten søppelsamler. Dette er mulig takket være *eierskapsmodellen*: kompilatoren sporer hvem som eier hver verdi, og frigjør minne automatisk når eieren går ut av scope. Ingen garbage collector, ingen manuell `free`.

## Typesystem

|  | **Statisk typet** | **Dynamisk typet** |
|---|---|---|
| **Strengt typet** | **Rust**, Haskell, Java, Kotlin | Python, Ruby |
| **Svakt typet** | C, C++ | JavaScript, PHP |

Rust er både statisk og strengt typet. C++ er statisk men *svakt* typet — det tillater implisitte konverteringer, `void*` og `reinterpret_cast`. Python er dynamisk men strengt (ingen implisitt typetvang). JavaScript er både dynamisk og svakt (`"5" + 3 === "53"`).

## Abstraksjonsnivå

|  | **Nullkostnad-abstraksjoner** | **Abstraksjoner med runtime-kostnad** |
|---|---|---|
| **Høynivå-ergonomi** | **Rust** | Java, Python, Go, Haskell, Kotlin |
| **Lavnivå-ergonomi** | C, C++ | |

Rust gir deg høynivå-ergonomi — iteratorer, closures, pattern matching, generics — som kompileres til like effektiv maskinkode som håndskrevet C. Dette kalles *nullkostnad-abstraksjoner*. De fleste høynivåspråk betaler for sine abstraksjoner med søppelsamler, virtuell dispatch eller JIT-kompilering. C og C++ er lette, men tilbyr færre ergonomiske abstraksjoner ut av boksen.

## Null-sikkerhet

|  | **Null finnes i språket** | **Null finnes ikke** |
|---|---|---|
| **Kompilator håndhever håndtering** | Kotlin, Swift | **Rust**, Haskell |
| **Kompilator håndhever ikke** | C, C++, Java, Go, JavaScript | |

Rust har ikke `null`. I stedet bruker det `Option<T>`, som enten er `Some(verdi)` eller `None`. Kompilatoren tvinger deg til å håndtere begge variantene — du kan ikke «glemme» å sjekke for manglende verdier.

## C++ vs Rust

| Egenskap | C++ | Rust |
|----------|-----|------|
| Ytelse | Høy (ingen GC) | Høy (ingen GC) |
| Minnesikkerhet | Ingen garanti | Garantert av kompilator |
| Null-sikkerhet | `nullptr` uten garanti | `Option<T>` med kompilatorsjekk |
| Trådsikkerhet | Programmererens ansvar | Garantert av kompilator |
| Feilhåndtering | Unntak (`throw`/`catch`) | `Result<T, E>` med `?`-operator |

## Oppsummering

Rust flytter feil fra kjøretid til kompileringstid. Mye av det dere lærer om minnefeil, dangling pointers og data races i C++ er nettopp det Rust er designet for å forhindre — ikke gjennom konvensjoner, men gjennom språkets typesystem.
