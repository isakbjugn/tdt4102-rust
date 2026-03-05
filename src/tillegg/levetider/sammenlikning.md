# Sammenlikning

## Levetidsannotasjoner

| Egenskap | C++ | Rust |
|----------|-----|------|
| [Levetider](../../ordliste.md#levetid) | Implisitte, ikke verifisert | Sporet av kompilatoren |
| Annotasjonssyntaks | *(finnes ikke)* | `'a`, `'static` |
| [Levetidselisjon](../../ordliste.md#levetidselisjon) | *(ikke relevant)* | Tre automatiske regler |
| Referanser i strukturer | Ingen spesiell syntaks | Krever [levetidsannotering](../../ordliste.md#levetidsannotering) |

## Viktige forskjeller

**Levetidsannotasjoner er sjelden nødvendige.** [Elisjonsreglene](../../ordliste.md#levetidselisjon) dekker de aller fleste funksjoner automatisk. I praksis trenger du bare eksplisitte annotasjoner når en funksjon returnerer en referanse som kan komme fra flere ulike inputparametere, eller når en struct lagrer en referanse. Dette gjør at levetidssystemet sjelden er i veien i daglig koding — men det er alltid der som sikkerhetsnett.

For en sammenlikning av låneregler, referansevaliditet og dangling references, se [sammenlikningssiden i kapitlet om lån og referanser](../../laan_og_referanser/sammenlikning.md).
