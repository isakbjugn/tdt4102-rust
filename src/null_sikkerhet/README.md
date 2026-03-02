# Null-sikkerhet

Null-pekere er en av de vanligste kildene til krasj og [udefinert oppførsel](../ordliste.md#udefinert-oppforsel) i C++. Problemet er så utbredt at oppfinneren av `null`, Tony Hoare, kalte det «the billion-dollar mistake».

Rust løser dette fundamentalt: språket har ikke `null` i det hele tatt. I stedet bruker Rust typene `Option<T>` og `Result<T, E>`, som tvinger deg til å håndtere fraværet av en verdi *før* du kan bruke den. Feil som i C++ gir krasj ved kjøretid, blir i Rust til [kompileringsfeil](../ordliste.md#kompileringsfeil).

Dette kapittelet viser først problemene med `null` i C++, deretter hvordan Rust løser dem, og avslutter med en sammenlikning der vi også ser på C++ sine nyere alternativer (`std::optional` og `std::expected`).
