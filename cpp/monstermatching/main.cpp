#include <iostream>
#include <map>
#include <string>
#include <variant>
#include <tuple>

// ANCHOR: switch_begrensninger
void kategoriser_med_switch(int tall) {
    switch (tall) {
        case 1:
            std::cout << "En" << std::endl;
            break;
        case 2:
            std::cout << "To" << std::endl;
            break;
        default:
            std::cout << "Noe annet" << std::endl;
            break;
    }
}

// switch fungerer ikke med std::string:
// void sjekk_navn(std::string navn) {
//     switch (navn) { ... } // Kompileringsfeil!
// }

// switch kan heller ikke destrukturere sammensatte typer:
// switch (std::make_pair(1, 2)) { ... } // Kompileringsfeil!
// ANCHOR_END: switch_begrensninger

// ANCHOR: if_else_kjede
void kategoriser_figur(const std::string& type, double a, double b, double c) {
    double areal;
    if (type == "sirkel") {
        areal = 3.14159 * a * a;
    } else if (type == "rektangel") {
        areal = a * b;
    } else if (type == "trekant") {
        // Herons formel
        double s = (a + b + c) / 2.0;
        areal = std::sqrt(s * (s - a) * (s - b) * (s - c));
    } else {
        std::cout << "Ukjent figur: " << type << std::endl;
        return;
    }
    std::cout << type << ": areal = " << areal << std::endl;
}
// ANCHOR_END: if_else_kjede

// ANCHOR: variant_grunnleggende
void variant_eksempel() {
    // std::variant kan holde en av flere typer
    std::variant<int, double, std::string> verdi;

    verdi = 42;
    std::cout << "int: " << std::get<int>(verdi) << std::endl;

    verdi = 3.14;
    std::cout << "double: " << std::get<double>(verdi) << std::endl;

    verdi = std::string("hei");
    std::cout << "string: " << std::get<std::string>(verdi) << std::endl;

    // Hvilken type er aktiv?
    std::cout << "Aktiv index: " << verdi.index() << std::endl; // 2 (string)
}
// ANCHOR_END: variant_grunnleggende

// ANCHOR: visit_overloaded
// Overloaded-monsteret: kombinerer flere lambdaer til en visitor
template<class... Ts>
struct Overloaded : Ts... { using Ts::operator()...; };

void visit_eksempel() {
    std::variant<int, double, std::string> verdi = std::string("hei");

    std::visit(Overloaded{
        [](int i)                { std::cout << "Heltall: " << i << std::endl; },
        [](double d)             { std::cout << "Desimaltall: " << d << std::endl; },
        [](const std::string& s) { std::cout << "Tekst: " << s << std::endl; },
    }, verdi);

    // Fungerer ogsa med returverdi
    verdi = 42;
    int resultat = std::visit(Overloaded{
        [](int i)                { return i * 2; },
        [](double d)             { return static_cast<int>(d); },
        [](const std::string& s) { return static_cast<int>(s.size()); },
    }, verdi);
    std::cout << "Resultat: " << resultat << std::endl; // 84
}
// ANCHOR_END: visit_overloaded

// ANCHOR: get_og_get_if
void get_eksempler() {
    std::variant<int, double, std::string> verdi = 42;

    // std::get<T>() - kaster std::bad_variant_access hvis feil type
    std::cout << std::get<int>(verdi) << std::endl; // OK: 42
    // std::get<double>(verdi); // Kaster unntak!

    // std::get_if<T>() - returnerer peker (nullptr hvis feil type)
    if (auto* p = std::get_if<int>(&verdi)) {
        std::cout << "Er int: " << *p << std::endl;
    }
    if (auto* p = std::get_if<double>(&verdi)) {
        std::cout << "Er double: " << *p << std::endl;
    } else {
        std::cout << "Er ikke double" << std::endl;
    }
}
// ANCHOR_END: get_og_get_if

// ANCHOR: strukturerte_bindinger
void strukturerte_bindinger() {
    // Destrukturering av tupler
    auto [x, y] = std::make_tuple(3.0, 4.0);
    double avstand = std::sqrt(x * x + y * y);
    std::cout << "Avstand: " << avstand << std::endl;

    // Destrukturering av par fra std::map
    std::map<std::string, int> poeng = {{"Ola", 95}, {"Kari", 88}};
    for (const auto& [navn, score] : poeng) {
        std::cout << navn << ": " << score << std::endl;
    }

    // Begrensning: kun toppniva - ingen nestede monstre
    // auto [[a, b], c] = ...; // Kompileringsfeil!
}
// ANCHOR_END: strukturerte_bindinger

int main() {
    std::cout << "--- switch-begrensninger ---" << std::endl;
    kategoriser_med_switch(1);
    kategoriser_med_switch(42);

    std::cout << "\n--- if/else if-kjede ---" << std::endl;
    kategoriser_figur("sirkel", 5.0, 0, 0);
    kategoriser_figur("rektangel", 4.0, 6.0, 0);
    kategoriser_figur("trekant", 3.0, 4.0, 5.0);

    std::cout << "\n--- std::variant ---" << std::endl;
    variant_eksempel();

    std::cout << "\n--- std::visit + Overloaded ---" << std::endl;
    visit_eksempel();

    std::cout << "\n--- std::get / std::get_if ---" << std::endl;
    get_eksempler();

    std::cout << "\n--- Strukturerte bindinger ---" << std::endl;
    strukturerte_bindinger();

    return 0;
}
