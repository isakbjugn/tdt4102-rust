# Smartpekere i C++

## Problemet: manuell minnehåndtering

Vi så i forrige kapittel at `new`/`delete` krever disiplin. En vanlig felle er [minnelekkasjer](../ordliste.md#minnelekkasje) ved tidlig retur:

```cpp
{{#include ../../cpp/smartpekere/main.cpp:lekkasje_early_return}}
```

Hvis `tidlig_retur` er `true`, returnerer funksjonen uten å kalle `delete` — minnet lekker. [Smartpekere](../ordliste.md#smartpeker) løser dette ved å knytte frigjøring til [RAII](../ordliste.md#raii).

## Historien: fra `auto_ptr` til moderne smartpekere

C++98 introduserte `auto_ptr` — den første smartpekeren i standardbiblioteket:

```cpp
// Pseudokode — auto_ptr er fjernet i C++17
std::auto_ptr<int> p(new int(42));
std::auto_ptr<int> q = p; // "Kopiering" overfører eierskap — p er nå ugyldig!
```

Problemet var at `auto_ptr` overførte eierskap ved *kopiering*. Det brøt med forventningen om at en kopi er uavhengig av originalen, og førte til subtile feil — spesielt med standardbibliotekcontainere som kopierer elementer internt.

`auto_ptr` ble markert som *deprecated* i C++11 (da `unique_ptr` tok over med eksplisitt [move-semantikk](../ordliste.md#move-semantikk)) og fjernet i C++17.

## `unique_ptr` — eksklusivt eierskap

`unique_ptr` eier ressursen eksklusivt. Den kan ikke kopieres, bare flyttes:

```cpp
{{#include ../../cpp/smartpekere/main.cpp:unique_ptr_grunnleggende}}
```

Eierskap kan overføres med `std::move`:

```cpp
{{#include ../../cpp/smartpekere/main.cpp:unique_ptr_move}}
```

`unique_ptr` er den foretrukne smartpekeren i C++ — bruk den med mindre du faktisk trenger delt eierskap.

## `shared_ptr` — delt eierskap

`shared_ptr` bruker [referansetelling](../ordliste.md#referansetelling) for å tillate flere eiere av samme ressurs. Minnet frigjøres når den siste `shared_ptr` går ut av [scope](../ordliste.md#scope):

```cpp
{{#include ../../cpp/smartpekere/main.cpp:shared_ptr_grunnleggende}}
```

## `weak_ptr` — svak referanse

`weak_ptr` observerer en `shared_ptr` uten å øke referansetelleren. Hovedbruksområdet er å bryte sykliske referanser:

```cpp
{{#include ../../cpp/smartpekere/main.cpp:weak_ptr_syklus}}
```

Her bruker `forrige` en `weak_ptr` i stedet for `shared_ptr`. Uten `weak_ptr` ville A og B holde hverandre i live via sirkulære sterke referanser, og minnet ville aldri bli frigjort.

For å bruke verdien fra en `weak_ptr` må du kalle `.lock()`, som returnerer en `shared_ptr` (eller `nullptr` hvis objektet er frigjort).

## `make_unique` og `make_shared`

Foretrekk `make_unique` og `make_shared` fremfor å bruke `new` direkte:

```cpp
{{#include ../../cpp/smartpekere/main.cpp:make_unique_eksempel}}
```

Fordelene er:
- **Unntakssikkerhet** — unngår minnelekkasje hvis et unntak kastes mellom `new` og konstruksjon av smartpekeren
- **Lesbarhet** — typen skrives bare én gang
- **Ytelse** (for `make_shared`) — allokerer kontrollblokk og objekt i ett kall

## Vanlige fallgruver

1. **Sykliske `shared_ptr`** — to objekter som holder `shared_ptr` til hverandre frigjøres aldri. Bruk `weak_ptr` for å bryte syklusen.
2. **Blanding av rå pekere og smartpekere** — å lage to `shared_ptr` fra samme rå peker gir [double free](../ordliste.md#double-free).
3. **`unique_ptr` som `nullptr`** — etter en `std::move` er den opprinnelige pekeren `nullptr`. Å bruke den er [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).
