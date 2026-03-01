# Sammenlikning

## Samme mønster, ulik mekanisme

Både C++ og Rust bygger på [RAII](../ordliste.md#raii) som grunnleggende ressurshåndteringsmønster. Forskjellen ligger i *mekanismen* og i hvor mye kompilatoren hjelper deg å bruke mønsteret riktig.

| Egenskap | C++ | Rust |
|----------|-----|------|
| RAII-mekanisme | [Destruktør](../ordliste.md#destruktor) (`~Klasse()`) | [`Drop`](../ordliste.md#drop)-trait |
| Manuell tidlig frigjøring | Mulig (men fragilt) | `std::mem::drop(verdi)` |
| Kalle destruktør direkte | Mulig (`obj.~Klasse()`) | Ikke tillatt |
| Rekkefølge (lokale variabler) | Omvendt (LIFO) | Omvendt (LIFO) |
| Rekkefølge (felter) | Deklarasjonsrekkefølge | Deklarasjonsrekkefølge |
| Filhåndtering | `std::ofstream` | `std::fs::File` |
| Låser | `std::lock_guard` | `MutexGuard` |

## Viktige forskjeller

**Rust nekter bruk etter `drop()`.** Når du kaller `drop(verdi)` i Rust, tar funksjonen [eierskap](../ordliste.md#eierskap) over verdien. Kompilatoren nekter deg å bruke den etterpå — det er en [kompileringsfeil](../ordliste.md#kompileringsfeil). I C++ kan du kalle destruktøren manuelt (`obj.~Klasse()`) og deretter fortsette å bruke objektet — noe som gir [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

**Du kan ikke kalle `.drop()` direkte.** I Rust er det forbudt å kalle `verdi.drop()` — kompilatoren gir feilmelding. Grunnen er at Rust alltid kaller `drop()` automatisk når verdien går ut av [scope](../ordliste.md#scope), og et manuelt kall ville gi [double free](../ordliste.md#double-free). I stedet bruker du `std::mem::drop()`, som er en vanlig funksjon som tar eierskap og lar verdien gå ut av scope umiddelbart.

**Mutex tvinger RAII i Rust.** I C++ *kan* du låse og låse opp en mutex manuelt — `lock_guard` er en bekvemmelighet, ikke et krav. I Rust er `MutexGuard` den *eneste* måten å få tilgang til dataen inni en `Mutex`. Du kan ikke glemme å bruke RAII-mønsteret fordi typesystemet krever det.

**Destruktørrekkefølgen er lik.** Både C++ og Rust dropper lokale variabler i omvendt deklarasjonsrekkefølge og felter i deklarasjonsrekkefølge. Denne likheten er bevisst — det sikrer at avhengigheter mellom variabler alltid er gyldige under opprydding.
