# Mønstermatching

I forrige kapittel så vi `match` og `if let` brukt på `Option` og `Result` for å håndtere [null-sikkerhet](../ordliste.md#null-sikkerhet). Men [mønstermatching](../ordliste.md#monstermatching) er et langt bredere konsept — hva om vi har egendefinerte [sumtyper](../ordliste.md#sumtype) med flere varianter? Og hvordan kan vi bruke [destrukturering](../ordliste.md#destrukturering) til å trekke ut data fra sammensatte verdier?

Rust har et kraftig system for mønstermatching med [uttømmende sjekk](../ordliste.md#uttommende-sjekk), sjekker (eng. *match guards*) og destrukturering på alle nivåer. C++ tilbyr `switch` for heltall og `std::variant` med `std::visit` (C++17) for type-sikre unioner — men med mer begrenset syntaks og ingen kompilatorgaranti for uttømmenhet.

Dette kapittelet viser hvordan de to språkene håndterer forgreining basert på datastruktur, og sammenligner uttrykkskraften i hvert språk.
