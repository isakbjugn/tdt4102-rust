# Sammenlikning

Tabellen under oppsummerer de viktigste forskjellene mellom C++ og Rust når det gjelder [minnehåndtering](../ordliste.md#minnehandtering):

| C++ | Rust |
|-----|------|
| `new int(42)` | `Box::new(42)` |
| `delete p` | Automatisk ved [scope](../ordliste.md#scope)-slutt |
| [Minnelekkasje](../ordliste.md#minnelekkasje) mulig | Umulig (uten `unsafe`) |
| [Use after free](../ordliste.md#use-after-free) mulig | [Kompileringsfeil](../ordliste.md#kompileringsfeil) |

## Address Sanitizer

For å fange minnefeil i C++ kan du bruke Address Sanitizer (ASan):

```bash
clang++ -std=c++20 -fsanitize=address -g cpp/minnehandtering/main.cpp -o main && ./main
```

ASan gir tydelige feilmeldinger for use-after-free, buffer overflow, og andre minnefeil — men kun ved kjøretid, ikke kompilering. Rusts kompilator fanger de fleste av disse feilene allerede ved kompilering.
