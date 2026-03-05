# `RefCell<T>` og indre mutabilitet

> **Merk:** Denne siden forutsetter kjennskap til Rusts [låneregler](../ordliste.md#laaneregler) og [lånesjekkeren](../ordliste.md#laanesjekkeren). Den er ment som fordypning for dem som har erfaring med Rusts referansesystem og ønsker å forstå hvordan man kan bryte lånereglene kontrollert.

## Problemet

Normalt sjekker Rust [lånereglene](../ordliste.md#laaneregler) ved kompilering: enten én muterbar referanse, eller flere uforanderlige. Men noen ganger trenger du å mutere en verdi gjennom en uforanderlig referanse — for eksempel når du har delt eierskap med `Rc<T>` og likevel vil endre verdien.

## `RefCell<T>` — låneregler ved kjøretid

`RefCell<T>` flytter lånesjekken fra kompilering til kjøretid, noe som muliggjør [indre mutabilitet](../ordliste.md#indre-mutabilitet):

```rust
# use std::cell::RefCell;
{{#include ../../rust/src/smartpekere/mod.rs:refcell_grunnleggende}}
```

Hvis du bryter lånereglene ved kjøretid (f.eks. to muterbare lån samtidig), panicker programmet i stedet for å gi [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

## `Rc<RefCell<T>>` — delt eierskap med mutering

`RefCell<T>` kombineres ofte med `Rc<T>` som `Rc<RefCell<T>>` — delt [eierskap](../ordliste.md#eierskap) med mulighet for mutering. Dette er et vanlig mønster når flere deler av programmet trenger å eie og mutere samme verdi.

## Sammenlikning med C++

`RefCell<T>` har ingen direkte C++-ekvivalent. I C++ kan du alltid mutere gjennom en peker (const-correctness er rådgivende, ikke håndhevet). Rust krever `RefCell` for å bryte de vanlige lånereglene — og gir deg en kjøretidssjekk i bytte.

## Når bruker du `RefCell<T>`?

| Behov | Type |
|-------|------|
| Mutering gjennom delt referanse | `RefCell<T>` |
| Delt + muterbar (én tråd) | `Rc<RefCell<T>>` |
