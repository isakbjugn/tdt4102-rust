# Oppgaver: Minnehåndtering

## Oppgave 1: Enkel allokering

Skriv kode som:
1. Allokerer et heltall på heapen med verdien 100
2. Skriver ut verdien
3. Frigjør minnet

## Oppgave 2: Array-allokering

Skriv kode som:
1. Allokerer en array med 5 heltall på heapen
2. Fyller arrayen med verdiene 1-5
3. Skriver ut alle verdiene
4. Frigjør minnet korrekt

Spørsmål: Hva skjer hvis du bruker `delete` i stedet for `delete[]`?

## Oppgave 3: Finn feilen

Følgende kode har en minnelekkasje. Hvor er den, og hvordan fikser du den?

```cpp
void prosesser_data(bool skal_avbryte) {
    int* data = new int[1000];

    if (skal_avbryte) {
        std::cout << "Avbryter tidlig" << std::endl;
        return;
    }

    // ... prosesser data ...

    delete[] data;
}
```

## Oppgave 4: Double free

Hva skjer når denne koden kjøres? Hvorfor?

```cpp
int* p = new int(42);
int* q = p;
delete p;
delete q;
```

## Bonusoppgave

Skriv en funksjon som demonstrerer "use after free" - altså bruk av en peker etter at minnet er frigjort. Hva skriver programmet ut? Er resultatet forutsigbart?
