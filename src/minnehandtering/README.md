# Minnehåndtering

I C++ har du direkte kontroll over [minnehåndtering](../ordliste.md#minnehandtering): du allokerer med `new` og frigjør med `delete`. Dette gir fleksibilitet, men også risiko for feil som [minnelekkasjer](../ordliste.md#minnelekkasje) og [udefinert oppførsel](../ordliste.md#udefinert-oppforsel).

Rust løser dette med [eierskapsmodellen](../ordliste.md#eierskap): hver verdi har nøyaktig én eier, og minnet frigjøres automatisk når eieren går ut av scope. Feil som i C++ først viser seg ved kjøretid, blir i Rust til [kompileringsfeil](../ordliste.md#kompileringsfeil).

Dette kapittelet dekker begge tilnærminger — først C++, deretter Rust — og avslutter med en sammenlikning.
