# Levetider

I [kapitlet om lån og referanser](../../laan_og_referanser/README.md) så vi hvordan [lånesjekkeren](../../ordliste.md#laanesjekkeren) verifiserer at referanser ikke overlever dataen de peker på, og at [lånereglene](../../ordliste.md#laaneregler) forebygger datakapløp.

I de fleste tilfeller klarer kompilatoren å utlede [levetider](../../ordliste.md#levetid) automatisk. Men noen ganger — spesielt når en funksjon returnerer en referanse som kan stamme fra flere inputparametere, eller når en struct lagrer en referanse — trenger kompilatoren hjelp. Da bruker vi [levetidsannotasjoner](../../ordliste.md#levetidsannotering).

Dette tillegget dekker levetidsannotasjoner, elisjonsregler, levetider i strukturer og `'static`-levetiden.
