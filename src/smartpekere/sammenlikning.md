# Sammenlikning

## Smartpekere side om side

| C++ | Rust | Bruksområde |
|-----|------|-------------|
| `unique_ptr<T>` | `Box<T>` | Eksklusivt eierskap, heap-allokering |
| `shared_ptr<T>` | `Rc<T>` / `Arc<T>` | Delt eierskap med [referansetelling](../ordliste.md#referansetelling) |
| `weak_ptr<T>` | `Weak<T>` | Svak referanse, bryte sykluser |
| *(ingen direkte ekvivalent)* | `RefCell<T>` | [Indre mutabilitet](../ordliste.md#indre-mutabilitet) |

## Hva C++ kan lære av Rust

- **Foretrekk `unique_ptr`** — akkurat som Rust bruker enkel eierskap som standard, bør C++-kode bruke `unique_ptr` med mindre delt eierskap er nødvendig.
- **Unngå rå pekere for eierskap** — bruk rå pekere bare for ikke-eiende observasjon, aldri for å uttrykke eierskap.
- **Tenk på trådsikkerhet** — Rust skiller mellom `Rc` (ikke-trådsikker) og `Arc` (trådsikker). I C++ er `shared_ptr` trådsikker for selve kontrollblokken, men *ikke* for verdien den peker på.
- **Sykliske referanser** — i begge språk må du bruke svake referanser for å bryte sykluser. Forskjellen er at Rust gjør det vanskeligere å havne i situasjonen i utgangspunktet, fordi delt eierskap krever eksplisitt opt-in med `Rc`/`Arc`.

## Viktige forskjeller i praksis

**`Box<T>` kan aldri være null.** I C++ kan en `unique_ptr` være `nullptr` (og er det etter en `std::move`). I Rust har `Box<T>` alltid en gyldig verdi — [null-sikkerhet](../ordliste.md#null-sikkerhet) er garantert av typesystemet.

**Låneregler eliminerer mange smartpeker-behov.** Mye av det C++ bruker `shared_ptr` til (f.eks. å sende data til flere funksjoner) løses i Rust med vanlige referanser (`&T` / `&mut T`). Du trenger bare `Rc`/`Arc` når eierskapet faktisk må deles.

**`RefCell<T>` har ingen direkte C++-ekvivalent.** I C++ kan du alltid mutere gjennom en peker (const-correctness er rådgivende, ikke håndhevet). Rust krever `RefCell` for å bryte de vanlige lånereglene — og gir deg en runtime-sjekk i bytte.

**Move-semantikk er eksplisitt i C++, implisitt i Rust.** I C++ forblir en `unique_ptr` gyldig (som `nullptr`) etter `std::move`, og å bruke den er lovlig men farlig. I Rust er en flyttet `Box` utilgjengelig — kompilatoren nekter å la deg bruke den.
