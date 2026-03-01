# RAII i C++

## Prinsippet

[RAII](../ordliste.md#raii) er et designmønster der en konstruktør erverver en ressurs og [destruktøren](../ordliste.md#destruktor) frigjør den. Fordi destruktøren kalles automatisk når objektet går ut av [scope](../ordliste.md#scope), kan ressurser aldri glemmes — uavhengig av hvordan kontrollflyten forlater scopet.

Mønsteret gjelder langt mer enn [minne](../ordliste.md#minnehandtering): filer, låser, nettverkstilkoblinger og databasetransaksjoner følger alle samme prinsipp.

## Filhåndtering uten RAII

Med C-stilens `fopen`/`fclose` er det lett å lekke filhåndtak:

```cpp
{{#include ../../cpp/raii/main.cpp:raii_fil_uten_raii}}
```

Hvis `feil` er `true`, returnerer funksjonen uten å lukke filen. Filhåndtaket lekker.

## Filhåndtering med RAII

`std::ofstream` wrapper filhåndtaket i et RAII-objekt. Destruktøren lukker filen automatisk:

```cpp
{{#include ../../cpp/raii/main.cpp:raii_fil_med_raii}}
```

Uansett hvordan funksjonen avslutter — normalt, ved tidlig retur, eller ved unntak — lukkes filen.

## Custom RAII-klasse: `LoggFil`

Du kan bygge egne RAII-wrappere. Her er en enkel loggfilklasse der konstruktøren åpner filen og destruktøren lukker den:

```cpp
{{#include ../../cpp/raii/main.cpp:raii_custom_loggfil}}
```

Når en `LoggFil` går ut av scope, skrives avslutningslinjen og filen lukkes — uten at brukeren av klassen trenger å tenke på det.

## `lock_guard` — RAII for låser

Mutex-låsing er et annet klassisk RAII-bruksområde. `std::lock_guard` tar låsen i konstruktøren og slipper den i destruktøren:

```cpp
{{#include ../../cpp/raii/main.cpp:raii_lock_guard}}
```

Uten `lock_guard` måtte du husket å kalle `mtx.unlock()` — og en glemt `unlock` kan føre til deadlock.

## Destruktørrekkefølge

Lokale variabler destrueres i *omvendt* deklarasjonsrekkefølge (LIFO — sist inn, først ut):

```cpp
{{#include ../../cpp/raii/main.cpp:raii_destruktor_rekkefolge}}
```

Utskrift:
```text
  Oppretter: A
  Oppretter: B
  Oppretter: C
  Frigjør:   C
  Frigjør:   B
  Frigjør:   A
```

Dette er viktig fordi senere variabler kan avhenge av tidligere. Ved å destruere i omvendt rekkefølge sikrer C++ at avhengigheter alltid er gyldige under opprydding.

> **Merk:** For medlemsvariabler i en klasse er rekkefølgen også definert — de destrueres i omvendt *deklarasjonsrekkefølge* (ikke rekkefølgen i initialiseringslisten).
