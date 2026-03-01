# Livstider i C++

## Referanser og pekere

C++ har to mekanismer for å referere til eksisterende data uten å kopiere: *referanser* (`int&`) og *pekere* (`int*`). Begge gir deg tilgang til en verdi uten å ta [eierskap](../ordliste.md#eierskap) — men ingen av dem har noen innebygd garanti for at dataen de peker på fortsatt eksisterer.

[Livstiden](../ordliste.md#livstid) til en referanse eller peker er *implisitt*: kompilatoren verifiserer ikke at referansen er gyldig gjennom hele brukstiden. Det er programmererens ansvar å sikre at referanser aldri overlever dataen de peker på. Bryter du denne regelen, får du [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

## Dangling reference ved retur

Den enkleste feilen er å returnere en referanse til en lokal variabel:

```cpp
{{#include ../../cpp/livstider/main.cpp:livstid_dangling_retur}}
```

Variabelen `lokal` lever på [stacken](../ordliste.md#stack) og destrueres når funksjonen returnerer. Referansen som returneres peker på frigjort minne — en klassisk [dangling pointer](../ordliste.md#dangling-pointer).

De fleste kompilatorer gir en advarsel her (`-Wreturn-local-addr`), men det er *bare* en advarsel — koden kompilerer likevel.

## Dangling peker etter scope

En mer subtil variant er å ta adressen til en variabel i en indre blokk:

```cpp
{{#include ../../cpp/livstider/main.cpp:livstid_dangling_scope}}
```

Pekeren `p` overlever blokken der `lokal` ble opprettet. Etter at blokken avsluttes, er `lokal` destruert og `*p` er udefinert oppførsel. Her gir kompilatoren typisk *ingen* advarsel.

## Ugyldig referanse etter omallokering

Det kanskje farligste tilfellet er iteratorinvalidering — en referanse inn i en datastruktur som omallokeres:

```cpp
{{#include ../../cpp/livstider/main.cpp:livstid_vektor_invalidering}}
```

`push_back` kan føre til at vektoren omallokerer sitt interne buffer. Når det skjer, peker `ref` på frigjort minne. Dette er [udefinert oppførsel](../ordliste.md#udefinert-oppforsel) som kan fungere tilfeldig i testing, men krasje i produksjon.

## Hvorfor kompilatoren ikke hjelper

C++-kompilatoren sporet ikke [livstider](../ordliste.md#livstid). Den har ingen modell for hvor lenge en referanse er gyldig i forhold til dataen den peker på. Noen verktøy kan hjelpe:

- **AddressSanitizer (ASan)** og **Valgrind** oppdager ugyldige minneaksesser — men bare ved *kjøretid*, og bare for kodestier som faktisk kjøres under testing.
- **Statiske analysatorer** (clang-tidy, PVS-Studio) fanger noen mønstre, men kan ikke dekke alle tilfeller.

Ingen av disse gir den *garantien* som en kompileringssjekk kan gi.
