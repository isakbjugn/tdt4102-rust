# Livstider

I de forrige kapitlene så vi hvordan [eierskap](../ordliste.md#eierskap) og [smartpekere](../ordliste.md#smartpeker) sørger for at minne frigjøres automatisk. Men hva skjer når du bare vil *[låne](../ordliste.md#laan)* en verdi — bruke den uten å ta eierskap?

I C++ er det programmererens ansvar å sørge for at referanser og pekere ikke overlever dataen de peker på. Bryter du denne regelen, får du [dangling pointers](../ordliste.md#dangling-pointer) og [udefinert oppførsel](../ordliste.md#udefinert-oppforsel) — feil som ofte først dukker opp ved kjøretid.

Rust løser dette med [lånesjekkeren](../ordliste.md#laanesjekkeren), som verifiserer ved kompilering at alle referanser har gyldige [livstider](../ordliste.md#livstid). Dette kapittelet ser på problemene som oppstår i C++, og hvordan Rusts livstidssystem forebygger dem.
