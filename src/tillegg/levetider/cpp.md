# Levetidsannotasjoner i C++

C++ har ikke noe konsept som tilsvarer Rusts [levetidsannotasjoner](../../ordliste.md#levetidsannotering). Det finnes ingen syntaks for å uttrykke sammenhenger mellom referansers [levetider](../../ordliste.md#levetid), og kompilatoren gjør ingen slik verifisering.

For en gjennomgang av problemene dette fører til — dangling references, pekere til frigjort minne og vektor-invalidering — se [C++-siden i kapitlet om lån og referanser](../../laan_og_referanser/cpp.md).
