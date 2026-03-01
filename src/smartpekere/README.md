# Smartpekere

I C++ ble [smartpekere](../ordliste.md#smartpeker) innført for å automatisere frigjøring av heap-allokert minne via [RAII](../ordliste.md#raii) — i stedet for å huske `delete` manuelt, lar du et objekt ta [eierskap](../ordliste.md#eierskap) til minnet og frigjøre det automatisk.

Rust trenger ikke smartpekere for grunnleggende minnehåndtering — eierskapsmodellen håndterer det. Men for scenarier der enkel eierskap ikke er nok, som delt eierskap, [referansetelling](../ordliste.md#referansetelling) eller [indre mutabilitet](../ordliste.md#indre-mutabilitet), tilbyr Rust egne smartpeker-typer.

Dette kapittelet dekker smartpekere i C++ og Rust, og avslutter med en sammenlikning.
