# Lån og referanser

I forrige kapittel så vi hvordan [eierskap](../ordliste.md#eierskap) og [move-semantikk](../ordliste.md#move-semantikk) sørger for at hver verdi har nøyaktig én eier. Men hva gjør du når du bare vil *bruke* en verdi uten å ta eierskap?

I C++ har du referanser (`&`) og pekere (`*`), men ingen kompilatorgaranti for at de er gyldige. Resultatet er [dangling pointers](../ordliste.md#dangling-pointer) og [udefinert oppførsel](../ordliste.md#udefinert-oppforsel) som først viser seg ved kjøretid.

Rust løser dette med et [lånesystem](../ordliste.md#laan) der [lånesjekkeren](../ordliste.md#laanesjekkeren) verifiserer ved kompilering at alle referanser overholder [lånereglene](../ordliste.md#laaneregler) og har gyldige [levetider](../ordliste.md#levetid). Dette kapittelet ser på problemene som oppstår i C++, og hvordan Rust forebygger dem.
