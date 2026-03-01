# RAII

I de forrige kapitlene så vi hvordan minne håndteres automatisk med [smartpekere](../ordliste.md#smartpeker) og [eierskap](../ordliste.md#eierskap). Men hva med *andre* ressurser — filer, låser, nettverkstilkoblinger? [RAII](../ordliste.md#raii) knytter enhver ressurs til et objekts levetid: [konstruktøren](../ordliste.md#destruktor) erverver ressursen, og [destruktøren](../ordliste.md#destruktor) frigjør den automatisk når objektet går ut av [scope](../ordliste.md#scope).

Både C++ og Rust bygger på RAII som grunnleggende mønster — men med ulike mekanismer. C++ bruker destruktører (`~Klasse()`), mens Rust bruker [`Drop`](../ordliste.md#drop)-traiten.

Dette kapittelet ser på RAII som generelt designmønster, med fokus på ikke-minneressurser, destruktørlogikk og eksplisitt frigjøring.
