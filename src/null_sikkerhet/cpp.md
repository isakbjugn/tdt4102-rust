# Null-sikkerhet i C++

I C++ er `nullptr` en helt lovlig verdi for pekere. Kompilatoren advarer deg ikke om at en peker kan være null, og dereferering av `nullptr` gir [udefinert oppførsel](../ordliste.md#udefinert-oppforsel). Dette gjør null-relaterte feil til en av de vanligste feilkildene i C++.

## Dereferering av `nullptr`

En funksjon som tar en peker har ingen garanti for at pekeren er gyldig:

```cpp
{{#include ../../cpp/null_sikkerhet/main.cpp:nullptr_deref}}
```

Kallet `skriv_lengde(nullptr)` kompilerer uten advarsler, men krasjer ved kjøretid. Kompilatoren hjelper deg ikke — ansvaret er helt og holdent ditt.

## Glemte null-sjekker

Funksjoner som returnerer pekere bruker ofte `nullptr` for å signalisere «ingen verdi». Det er lett å glemme sjekken:

```cpp
{{#include ../../cpp/null_sikkerhet/main.cpp:glemmer_null_sjekk}}
```

Kalleren *bør* sjekke om resultatet er `nullptr`, men ingenting tvinger dem til det:

```cpp
int tall[] = {1, 3, 5};
int* resultat = finn_partall(tall, 3);
std::cout << *resultat << std::endl; // UB — resultat er nullptr!
```

## Iteratorer som kan være ugyldige

Samme mønster dukker opp med `std::map::find`, som returnerer en iterator som kan være `end()`:

```cpp
{{#include ../../cpp/null_sikkerhet/main.cpp:map_find_null}}
```

Hvis studenten ikke finnes, er `it` lik `karakterer.end()`, og dereferering gir udefinert oppførsel. Igjen — ingen kompilatoradvarsel.

## «The billion-dollar mistake»

Tony Hoare, som introduserte null-referanser i 1965, har selv kalt det «my billion-dollar mistake»:

> *I call it my billion-dollar mistake. It was the invention of the null reference in 1965. [...] This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.*

Problemet er ikke at `null` finnes — det er at typesystemet ikke skiller mellom «denne pekeren kan være null» og «denne pekeren er alltid gyldig». Alt ser likt ut, og kompilatoren kan ikke hjelpe.

> Rust løser dette på språknivå — les videre i [Null-sikkerhet i Rust](./rust.md).
