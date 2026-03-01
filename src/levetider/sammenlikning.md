# Sammenlikning

## Implisitt vs. eksplisitt

| Egenskap | C++ | Rust |
|----------|-----|------|
| [Levetider](../ordliste.md#levetid) | Implisitte, ikke verifisert | Sporet av kompilatoren |
| Referansevaliditet | Programmererens ansvar | Garantert ved kompilering |
| [Dangling reference](../ordliste.md#dangling-pointer) | Kompilerer ([UB](../ordliste.md#udefinert-oppforsel)) | [Kompileringsfeil](../ordliste.md#kompileringsfeil) |
| Vektor-invalidering | Kompilerer (UB) | Kompileringsfeil |
| Annotasjonssyntaks | *(finnes ikke)* | `'a`, `'static` |
| [Levetidselisjon](../ordliste.md#levetidselisjon) | *(ikke relevant)* | Tre automatiske regler |
| Referanser i strukturer | Ingen spesiell syntaks | Krever [levetidsannotering](../ordliste.md#levetidsannotering) |
| Verktøy for validering | ASan, Valgrind (kjøretid) | [Lånesjekkeren](../ordliste.md#laanesjekkeren) (kompilering) |

## Viktige forskjeller

**Rust forebygger, C++ oppdager.** Rusts [lånesjekker](../ordliste.md#laanesjekkeren) fanger ugyldige referanser *før* programmet kjører — som en del av kompileringen. I C++ er de beste verktøyene (AddressSanitizer, Valgrind) kjøretidsbaserte: de finner bare feil i kodestier som faktisk utføres under testing, og de legger til betydelig overhead. Forskjellen er mellom en *garanti* og en *best-effort*-sjekk.

**[Lånereglene](../ordliste.md#laaneregler) forebygger datakapløp.** Regelen om «én `&mut` eller mange `&`» handler ikke bare om [levetider](../ordliste.md#levetid) — den forebygger også datakapløp i flertrådsprogrammer. Når kompilatoren garanterer at ingen kan lese data som noen andre skriver til, er en hel klasse av samtidige feil eliminert ved kompilering. I C++ krever dette eksplisitt synkronisering med mutexer, og feil er vanskelige å oppdage.

**Levetidsannotasjoner er sjelden nødvendige.** [Elisjonsreglene](../ordliste.md#levetidselisjon) dekker de aller fleste funksjoner automatisk. I praksis trenger du bare eksplisitte annotasjoner når en funksjon returnerer en referanse som kan komme fra flere ulike inputparametere, eller når en struct lagrer en referanse. Dette gjør at levetidssystemet sjelden er i veien i daglig koding — men det er alltid der som sikkerhetsnett.
