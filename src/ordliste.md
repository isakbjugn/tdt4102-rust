# Ordliste

| Term | Engelsk | Definisjon |
|------|---------|------------|
| Allokering | *Allocation* | Å reservere minne for en verdi, enten på stacken eller heapen. |
| Copy-semantikk | *Copy semantics* | Verdien kopieres bit-for-bit ved tilordning eller funksjonskall. I Rust gjelder dette typer som implementerer `Copy`-traiten, som primitive typer (`i32`, `f64`, `bool`). |
| Dangling pointer | *Dangling pointer* | En peker som refererer til minne som allerede er frigjort. I C++ gir dette udefinert oppførsel; i Rust er det en kompileringsfeil. |
| Double free | *Double free* | Å frigjøre samme minneområde to ganger. Gir udefinert oppførsel i C++. Rusts eierskapsmodell gjør dette umulig. |
| Eierskap | *Ownership* | Rusts grunnleggende modell for minnehåndtering. Hver verdi har nøyaktig én eier, og minnet frigjøres automatisk når eieren går ut av scope. |
| Heap | *Heap* | Minneområde for dynamisk allokering. Verdier på heapen overlever scopet de ble opprettet i, men må frigjøres — manuelt i C++ (`delete`), automatisk i Rust. |
| Kompileringsfeil | *Compile error* | En feil som oppdages av kompilatoren før programmet kjører. Rust bruker kompileringsfeil for å fange minnefeil som i C++ først viser seg ved kjøretid. |
| Livstid | *Lifetime* | Hvor lenge en referanse er gyldig. I C++ er livstider implisitte og ikke verifisert; i Rust kan kompilatoren kreve eksplisitte livstidsannotasjoner (`'a`) for å garantere at referanser aldri overlever dataen de peker på. |
| Minnehåndtering | *Memory management* | Prosessen med å allokere og frigjøre minne. C++ bruker manuell `new`/`delete`, Rust bruker eierskap og automatisk frigjøring. |
| Minnelekkasje | *Memory leak* | Allokert minne som aldri frigjøres. Vanlig i C++ når `delete` glemmes. I Rust forhindres dette av eierskapsmodellen (med unntak av bevisst bruk av `std::mem::forget` eller sykliske `Rc`-referanser). |
| Minnesikkerhet | *Memory safety* | Garanti for at et program ikke leser eller skriver til ugyldig minne. Rust garanterer minnesikkerhet ved kompilering; C++ har ingen slik garanti. |
| Move-semantikk | *Move semantics* | Overføring av eierskap til en verdi uten kopiering. I C++ krever dette eksplisitt `std::move`; i Rust er move standard oppførsel for typer som ikke implementerer `Copy`. |
| Null-sikkerhet | *Null safety* | Garanti for at en verdi aldri er `null` med mindre det er eksplisitt håndtert. Rust har ikke `null` — i stedet brukes `Option<T>`, som kompilatoren tvinger deg til å håndtere. |
| Nullkostnad-abstraksjon | *Zero-cost abstraction* | En abstraksjon som ikke medfører ekstra kjøretidskostnad sammenlignet med håndskrevet lavnivåkode. Rusts iteratorer, generics og traits er nullkostnad. |
| RAII | *Resource Acquisition Is Initialization* | Mønster der ressurser (minne, filhåndtak, låser) knyttes til et objekts levetid — ressursen frigjøres automatisk når objektet går ut av scope. Brukes i både C++ og Rust. |
| Scope | *Scope* | Området i koden der en variabel er gyldig. Når en variabel går ut av scope, frigjøres ressursene den eier (i både C++ med RAII og i Rust). |
| Smartpeker | *Smart pointer* | En peker som automatisk håndterer frigjøring av minne. I C++: `unique_ptr`, `shared_ptr`, `weak_ptr`. I Rust: `Box<T>`, `Rc<T>`, `Arc<T>`. |
| Søppelsamler | *Garbage collector (GC)* | En kjøretidsmekanisme som automatisk frigjør minne som ikke lenger er i bruk. Brukes i Java, Go, Python m.fl. Verken C++ eller Rust bruker søppelsamler. |
| Stack | *Stack* | Minneområde for lokale variabler. Allokering og frigjøring skjer automatisk og er svært rask. Begrenset størrelse. |
| Trait | *Trait* | Rusts mekanisme for å definere delt oppførsel, tilsvarende interfaces i Java eller concepts i C++20. Brukes også til å styre hvilke typer som kan kopieres (`Copy`), klones (`Clone`) osv. |
| Trådsikkerhet | *Thread safety* | Garanti for at data kan deles mellom tråder uten datakapløp. I Rust håndheves dette av typesystemet (`Send` og `Sync`-traits); i C++ er det programmererens ansvar. |
| Udefinert oppførsel | *Undefined behavior (UB)* | Oppførsel som ikke er spesifisert av språkstandarden. I C++ kan kompilatoren anta at UB aldri skjer, og optimalisere basert på dette — noe som kan gi uforutsigbare resultater. Rust har ingen UB i safe code. |
| Use after free | *Use after free* | Å bruke minne etter at det er frigjort. Gir udefinert oppførsel i C++. Rusts eierskapsmodell gjør dette til en kompileringsfeil. |
