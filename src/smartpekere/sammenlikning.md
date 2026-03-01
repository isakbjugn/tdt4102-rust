# Sammenlikning

## Ulike utgangspunkt

I C++ er [smartpekere](../ordliste.md#smartpeker) en *løsning på et problem*: manuell `new`/`delete` er feilutsatt, så `unique_ptr` og `shared_ptr` ble innført for å automatisere frigjøring via [RAII](../ordliste.md#raii). De er essensielle verktøy for sikker C++.

I Rust er [eierskapsmodellen](../ordliste.md#eierskap) innebygd i språket. Vanlige variabler har allerede eksklusivt eierskap med automatisk frigjøring — det `unique_ptr` gir C++, får du gratis i Rust. Rusts smartpekere dekker derfor mer *spesialiserte* behov:

| C++ | Rust | Merknad |
|-----|------|---------|
| `unique_ptr<T>` *(sikker eierskap)* | *(innebygd i eierskapsmodellen)* | Enhver variabel i Rust har eksklusivt eierskap med automatisk frigjøring |
| `unique_ptr<T>` *(heap-allokering)* | `Box<T>` | Brukes når du eksplisitt trenger heap, f.eks. rekursive typer eller store verdier |
| `shared_ptr<T>` | `Rc<T>` / `Arc<T>` | Delt eierskap med [referansetelling](../ordliste.md#referansetelling) |
| `weak_ptr<T>` | `Weak<T>` | Svak referanse, bryte sykluser |
| *(ingen direkte ekvivalent)* | `RefCell<T>` | [Indre mutabilitet](../ordliste.md#indre-mutabilitet) |

I C++ dekker `unique_ptr` altså to roller samtidig — sikker eierskap og heap-allokering — mens Rust deler dette i to: eierskapsmodellen (innebygd) og `Box<T>` (eksplisitt heap).

## Hva C++ kan lære av Rust

- **Eierskap som standard** — Rust viser at eksklusivt eierskap med automatisk frigjøring kan være *språkets standardoppførsel*, ikke noe du må velge å bruke. I C++ bør `unique_ptr` alltid foretrekkes fremfor rå `new`/`delete`.
- **Unngå rå pekere for eierskap** — bruk rå pekere bare for ikke-eiende observasjon, aldri for å uttrykke eierskap.
- **Tenk på trådsikkerhet** — Rust skiller mellom `Rc` (ikke-trådsikker) og `Arc` (trådsikker). I C++ er `shared_ptr` trådsikker for selve kontrollblokken, men *ikke* for verdien den peker på.
- **Sykliske referanser** — i begge språk må du bruke svake referanser for å bryte sykluser. Forskjellen er at Rust gjør det vanskeligere å havne i situasjonen i utgangspunktet, fordi delt eierskap krever eksplisitt opt-in med `Rc`/`Arc`.

## Viktige forskjeller i praksis

**`Box<T>` kan aldri være null.** I C++ kan en `unique_ptr` være `nullptr` (og er det etter en `std::move`). I Rust har `Box<T>` alltid en gyldig verdi — [null-sikkerhet](../ordliste.md#null-sikkerhet) er garantert av typesystemet.

**Låneregler eliminerer mange smartpeker-behov.** Mye av det C++ bruker `shared_ptr` til (f.eks. å sende data til flere funksjoner) løses i Rust med vanlige referanser (`&T` / `&mut T`). Du trenger bare `Rc`/`Arc` når eierskapet faktisk må deles.

**`RefCell<T>` har ingen direkte C++-ekvivalent.** I C++ kan du alltid mutere gjennom en peker (const-correctness er rådgivende, ikke håndhevet). Rust krever `RefCell` for å bryte de vanlige lånereglene — og gir deg en runtime-sjekk i bytte.

**Move-semantikk er eksplisitt i C++, implisitt i Rust.** I C++ forblir en `unique_ptr` gyldig (som `nullptr`) etter `std::move`, og å bruke den er lovlig men farlig. I Rust er en flyttet `Box` utilgjengelig — kompilatoren nekter å la deg bruke den.
