# Hvorfor er så mange interesserte i Rust?

Rust har i ni år på rad blitt kåret til det høyest beundrede programmeringsspråket blant brukerne på Stack Overflow.
I 2025 var det 72 % av Rust-utviklere som sa at de ville fortsette å bruke språket.

![](./img/stackoverflow-2025-admired-and-desired-top-13.png)

Les gjerne:

* [Why the developers who use Rust love it so much](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/)
* [7 Reasons to Use Rust for Your Next Development Project](https://geekflare.com/reasons-to-use-rust/)

## Hva er greia med Rust?

Rust er et systemprogrammeringsspråk som ble startet av Graydon Hoare hos Mozilla i 2010 og nådde versjon 1.0 i 2015.
Det er designet for å gi deg samme ytelse som C og C++, men med kompilatorgarantier for
[minnesikkerhet](ordliste.md#minnesikkerhet) — uten å bruke en [søppelsamler](ordliste.md#soppelsamler).

Rust er [multiparadigme](ordliste.md#paradigme): du kan skrive imperativ kode med mutabel tilstand, funksjonell kode med
iteratorer og [closures](ordliste.md#closure), og generisk kode med [traits](ordliste.md#trait) og typeparametere.
Syntaksen minner om C++, mens typesystemet er inspirert av Haskell og ML. Rust kompilerer til binærkode, og kan også
kompilere til [WebAssembly](https://webassembly.org/), som gjør det mulig å dra nytte av den høye ytelsen i
webapplikasjoner.

Rust er spesielt kjent for:

* **Null-sikkerhet**: Rust har ingen null-pekere, og håndterer i stedet fravær av verdi gjennom `Option<T>`, som tvinger
  utviklere til å eksplisitt håndtere tilfeller der en verdi kan være fraværende.

* **Minnesikkerhet**: Rust sikrer minnesikkerhet ved kompileringstid. Det gjør den ved å bruke en variabels _scope_.

* **Tråd-sikkerhet**: På grunn av hvordan Rust håndterer referanser til minne (gjennom konseptene _låning_ og
  _flytting_,
  som håndheves av _lånesystemet_ (eng. _borrow checker_) og variablers levetid, er Rust garantert å være tråd-sikkert.

* **Cargo**: Litt som _npm_ + _ESlint_ + _prettier_ + _Jest_. Tar seg av å kompilere koden, installere pakker, håndtere
  avhengigheter,
  lintsjekke koden din, kjøre tester. _Batterier inkludert_, med andre ord.

* **En hyggelig kompilator**: Rust-kompilatoren er spesielt velskreven, og kan ofte fortelle deg nøyaktig hvor i koden
  noe er feil, og kan foreslå hva du bør gjøre i stedet. (Cargo!)

> Kompileringssteget i Rust kan ofte oppfattes strengt, men gjennom å tvinge deg til å eksplisitt håndtere alt som kan
> feile, og å luke ut flere klasser av feil i kompileringssteget, gjør
> det at tiden til _debugging_ kuttes drastisk ned. Feilmeldingene fra kompilatoren gjør dette til en hyggelig oppgave.

## Hvem bruker Rust?

Rust brukes i økende grad av store selskaper, spesielt der ytelse og sikkerhet er viktig:

* **Linux-kjernen** godtar Rust-kode siden versjon 6.1 (2022), som det andre språket ved siden av C i kjernens over
  30-årige historie.[^linux]
* **Microsoft** bruker Rust i Windows-kjernen og Azure-infrastruktur for å redusere minnesikkerhetsfeil.[^microsoft]
* **Google** bruker Rust i Android, og rapporterte at andelen minnesikkerhetsfeil falt fra 76 % til 35 % etter
  innføringen.[^android] Rust brukes også i Chromium-prosjektet.[^chromium]
* **Amazon** bygde Firecracker, den Rust-baserte microVM-en som driver AWS Lambda og Fargate.[^firecracker]
* **Discord** byttet fra Go til Rust for ytelseskritiske tjenester, og oppnådde betydelig lavere latens og mer
  forutsigbar ytelse.[^discord]

[^linux]: [Rust in the Linux Kernel — The Linux Kernel documentation](https://docs.kernel.org/rust/)
[^microsoft]: [A proactive approach to more secure code — Microsoft Security Response Center](https://www.microsoft.com/en-us/msrc/blog/2019/07/a-proactive-approach-to-more-secure-code)
[^android]: [Memory Safe Languages in Android 13 — Google Security Blog](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)
[^chromium]: [Supporting the Use of Rust in Chromium — Google Security Blog](https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html)
[^firecracker]: [Firecracker – Open Source Secure Fast microVM for Serverless Computing — AWS Open Source Blog](https://aws.amazon.com/blogs/opensource/firecracker-open-source-secure-fast-microvm-serverless/)
[^discord]: [Why Discord is switching from Go to Rust — Discord Blog](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
