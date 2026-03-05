#include <iostream>
#include <map>
#include <optional>
#include <string>

// ANCHOR: nullptr_deref
void skriv_lengde(std::string* tekst) {
    // Ingen sjekk om tekst er nullptr!
    std::cout << "Lengde: " << tekst->size() << std::endl; // UB hvis nullptr
}
// ANCHOR_END: nullptr_deref

// ANCHOR: glemmer_null_sjekk
int* finn_partall(int* tall, int antall) {
    for (int i = 0; i < antall; i++) {
        if (tall[i] % 2 == 0) {
            return &tall[i];
        }
    }
    return nullptr; // Ingen partall funnet
}
// ANCHOR_END: glemmer_null_sjekk

// ANCHOR: map_find_null
void skriv_karakter(const std::map<std::string, char>& karakterer,
                    const std::string& student) {
    auto it = karakterer.find(student);
    // Hva om studenten ikke finnes i kartet?
    std::cout << student << " fikk " << it->second << std::endl; // UB!
}
// ANCHOR_END: map_find_null

// ANCHOR: optional_eksempel
std::optional<int> finn_foerste_partall(int* tall, int antall) {
    for (int i = 0; i < antall; i++) {
        if (tall[i] % 2 == 0) {
            return tall[i]; // Implisitt konvertert til std::optional<int>
        }
    }
    return std::nullopt; // Eksplisitt "ingen verdi"
}
// ANCHOR_END: optional_eksempel

// ANCHOR: expected_eksempel
#include <expected>

std::expected<int, std::string> del(int teller, int nevner) {
    if (nevner == 0) {
        return std::unexpected("Kan ikke dele på null");
    }
    return teller / nevner;
}
// ANCHOR_END: expected_eksempel

// ANCHOR: optional_monadisk
std::optional<int> partall_dobbel(int* tall, int antall) {
    return finn_foerste_partall(tall, antall)
        .transform([](int n) { return n * 2; }); // Som Rusts .map()
}

int partall_eller_null(int* tall, int antall) {
    return finn_foerste_partall(tall, antall)
        .value_or(0); // Som Rusts .unwrap_or()
}
// ANCHOR_END: optional_monadisk

int main() {
    // Eksempel 1: Dereferering av nullptr
    // skriv_lengde(nullptr); // Ville krasjet — UB

    // Eksempel 2: Glemmer null-sjekk
    int tall[] = {1, 3, 5};
    int* resultat = finn_partall(tall, 3);
    // Hva skjer her hvis det ikke finnes partall?
    // std::cout << *resultat << std::endl; // UB!

    // Eksempel 3: map::find uten sjekk
    std::map<std::string, char> karakterer = {{"Ola", 'A'}, {"Kari", 'B'}};
    // skriv_karakter(karakterer, "Per"); // UB — "Per" finnes ikke

    // Eksempel 4: std::optional
    int tall2[] = {1, 3, 5};
    auto opt = finn_foerste_partall(tall2, 3);
    if (opt.has_value()) {
        std::cout << "Fant partall: " << opt.value() << std::endl;
    } else {
        std::cout << "Ingen partall funnet" << std::endl;
    }

    // Eksempel 5: Monadiske operasjoner (C++23)
    int tall3[] = {1, 3, 5};
    auto dobbel = partall_dobbel(tall3, 3);
    std::cout << "Dobbel: " << dobbel.value_or(-1) << std::endl;

    int tall4[] = {1, 4, 7};
    auto dobbel2 = partall_dobbel(tall4, 3);
    std::cout << "Dobbel: " << dobbel2.value_or(-1) << std::endl;

    auto med_standard = partall_eller_null(tall3, 3);
    std::cout << "Med standard: " << med_standard << std::endl;

    return 0;
}
