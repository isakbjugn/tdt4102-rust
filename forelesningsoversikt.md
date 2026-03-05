# TDT4102 – Prosedyre- og objektorientert programmering: Forelesningsoversikt

Denne oversikten oppsummerer temaene dekket i forelesningene i TDT4102 ved NTNU. Kurset bruker C++ som hovedspråk og dekker grunnleggende programmering, objektorientering og minnehåndtering. Forelesning 09 er ikke tilgjengelig og er derfor utelatt.

---

## Forelesning 01 – Intro

**Hovedtemaer:** Kursinformasjon, introduksjon til C++

**Innhold:**
- Praktisk kursinformasjon (øvinger, eksamen, verktøy)
- C++-historie: Simula 67 → C → C++
- Hva C++ er: kompilert, lavnivåspråk med høynivåfunksjonalitet
- C++ vs. Python: ytelseforskjeller
- Språkversjoner fra C++98 til C++26
- Oppsett av VS Code som utviklingsmiljø
- Kompilering og kjøring av programmer
- Introduksjon til debugger

**Standardbibliotek:** `std::cout`, `std::endl`

---

## Forelesning 02 – Value Added Syntax

**Hovedtemaer:** Variabler, datatyper og grunnleggende syntaks

**Innhold:**
- Variabler og datatyper: `int`, `double`, `char`, `bool`, `string`
- `enum class` og `struct`/`class` (kort introduksjon)
- `void` som returtype
- Heltallsoverflow og konsekvenser
- Flyttallspresisjon og unøyaktighet
- Uinitialisert minne og udefinert oppførsel
- `std::println` (C++23)

**Standardbibliotek:** `std::cout`, `std::endl`, `std::string`, `std::println`

---

## Forelesning 03 – String Theory

**Hovedtemaer:** Kontrollflyt, funksjoner og strenger

**Innhold:**
- Input med `cin` og `getline`
- If-setninger og betingelser
- Løkker: `for`, `while`, `do-while`
- `break` og `continue`
- `switch`-setninger
- Funksjoner: deklarasjoner, definisjoner, rekursjon
- Standardparametere og funksjonsoverloading
- Call stack og debugging
- Strenger og strengoperasjoner
- AnimationWindow-grafikk
- Konvensjoner for navngivning

**Standardbibliotek:** `std::cin`, `std::cout`, `std::string`, `std::getline`

---

## Forelesning 04 – The Vector Enlistening

**Hovedtemaer:** Samlinger, tilfeldighet og grensekontroll

**Innhold:**
- Tilfeldig tallgenerering med moderne C++
- `std::vector` (dynamisk array) og `std::array` (statisk array)
- Tilgang med `.at()` vs. `[]` – grensekontroll
- Buffer overflow-sårbarheter og sikkerhet
- `const` for uforanderlige verdier
- `constexpr` for kompileringstidskonstanter
- Arbeid med terminalen

**Standardbibliotek:** `std::vector`, `std::array`, `std::random_device`, `std::default_random_engine`, `std::uniform_int_distribution`, `std::uniform_real_distribution`

---

## Forelesning 05 – Compilation Information

**Hovedtemaer:** Kompileringsprosessen og prosjektstruktur

**Innhold:**
- Funksjonsdeklarasjoner vs. definisjoner
- `#include`-direktivet og header-filer
- `#pragma once` for å unngå dobbel inkludering
- Kompileringsprosessen: kompilering og linking
- Linker-feil og kompilator-feil – feilsøking
- Kompilering fra kommandolinjen
- Arbeid med terminalen

**Standardbibliotek:** `<iostream>` (header)

---

## Forelesning 06 – Referential Treatment

**Hovedtemaer:** Referanser, parameterpassing og tegnkoding

**Innhold:**
- Referanser (`&`) i C++
- Pass-by-value vs. pass-by-reference vs. pass-by-const-reference
- Const-referanser for effektiv og trygg parameterpassing
- Structs: definisjon og bruk
- Tegnkoding: ASCII
- Strenger som sekvenser av tegn
- Konvensjoner for navngivning

**Standardbibliotek:** `std::string`, `std::vector` (brukt med referanser)

---

## Forelesning 07 – Object Orientation

**Hovedtemaer:** Objektorientert programmering

**Innhold:**
- Namespaces
- OOP vs. prosedyreorientert programmering
- `struct` vs. `class` i C++
- Synlighet: `public` og `private`
- Metoder og const-metoder
- Konstruktører og default-konstruktør
- `enum class` i OOP-kontekst

**Standardbibliotek:** `std::vector`, `std::array`, `std::string`

---

## Forelesning 08 – Files Advertising

**Hovedtemaer:** Filhåndtering, strømmer og operatoroverlasting

**Innhold:**
- Filsystem-konsepter og filbaner
- `std::filesystem`-biblioteket
- Lesing og skriving til filer
- Stringstreams for strengformatering
- Operatoroverlasting: `<<`, `>>`, `+`, med flere
- Friend-funksjoner

**Standardbibliotek:**
- Filsystem: `std::filesystem::path`, `std::filesystem::exists`, `std::filesystem::copy`, `std::filesystem::create_directories`, `std::filesystem::remove_all`, `std::filesystem::rename`, `std::filesystem::file_size`, `std::filesystem::absolute`, `std::filesystem::current_path`
- Strømmer: `std::ifstream`, `std::ofstream`, `std::stringstream`, `std::ostream`, `std::istream`

---

## *(Forelesning 09 – ikke tilgjengelig)*

---

## Forelesning 10 – Addressing References

**Hovedtemaer:** Minnehåndtering og pekere

**Innhold:**
- Minnehåndtering: RAM, allokering og deallokering
- Pekere: `*` (dereferering), `&` (adresse), `->` (medlemstilgang), `nullptr`
- Scope og levetid (lifetime) for variabler
- Stack vs. heap: `new` og `delete`
- Dangling pointers og risikoer
- Minnelekkasjer
- Kopikonstruktør
- Destruktorer

**Standardbibliotek:** Ingen nye – grunnleggende C++ minnehåndtering (`new`/`delete`)

---

## Forelesning 11 – Pointer Pointers

**Hovedtemaer:** Avansert pekerhåndtering, smartpekere og maps

**Innhold:**
- Minneadresser og pekeraritmetikk
- C-strenger (`char*` / `char[]`)
- Pekere og `const` (const-korrekthet)
- Minnehåndteringsrisikoer: lekkasjer, dangling references, double free
- Smartpekere: `unique_ptr` og `shared_ptr`
- `std::move` og flyttesemantikk
- Assosiative containere: maps

**Standardbibliotek:** `std::unique_ptr`, `std::shared_ptr`, `std::make_unique`, `std::make_shared`, `std::move`, `std::unordered_map`, `std::map`

---

## Spesialtemaer

### Null-sikkerhet

`nullptr` introduseres i forelesning 10 og 11 som erstatning for det gamle `NULL`-makroet. Null-sikkerhet som et eksplisitt konsept (slik det finnes i f.eks. Rust med `Option<T>` eller Kotlin med nullable types) behandles **ikke** i kurset. Det er ingen mekanisme for å garantere at en peker aldri er null på kompileringstidspunktet.

### Mønster-matching (pattern matching)

Mønster-matching i moderne forstand (slik som i Rust, Haskell eller C++23 `std::visit` med `std::variant`) dekkes **ikke**. Den nærmeste ekvivalenten er `switch`-setninger (forelesning 03), som kun fungerer på heltall og enum-verdier.

### Minnehåndtering og minnesikkerhet

Grundig dekket i forelesning 10 og 11. Temaene inkluderer:
- Manuell allokering/deallokering med `new`/`delete`
- Stack vs. heap
- Vanlige feil: minnelekkasjer, dangling pointers, double free
- Grensekontroll med `.at()` vs. `[]` (forelesning 04)
- Buffer overflow som sikkerhetsproblem (forelesning 04)

### RAII (Resource Acquisition Is Initialization)

RAII-prinsippet praktiseres **implisitt** gjennom destruktorer (forelesning 10) og smartpekere (forelesning 11), men selve begrepet «RAII» nevnes **ikke** eksplisitt. Destruktorer frigjør ressurser automatisk når objekter går ut av scope, noe som er kjernen i RAII-mønsteret.

### Smartpekere

Dekket i forelesning 11:
- `std::unique_ptr` – eksklusivt eierskap, kan ikke kopieres
- `std::shared_ptr` – delt eierskap med referansetelling
- `std::make_unique` og `std::make_shared` for sikker opprettelse
- `std::move` for overføring av eierskap

`std::weak_ptr` nevnes ikke.

### Levetider (lifetimes)

Scope og levetid for variabler dekkes i forelesning 10 (stack-variabler dør når de går ut av scope, heap-variabler lever til `delete` kalles). Dette er **ikke** det samme som Rusts eksplisitte lifetime-annotasjoner (`'a`). C++ har ingen tilsvarende kompileringstidskontroll av levetider – ansvaret ligger hos programmereren.
