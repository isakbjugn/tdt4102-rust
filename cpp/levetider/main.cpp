#include <iostream>
#include <vector>

// ANCHOR: levetid_dangling_retur
int& hent_verdi() {
    int lokal = 42;
    return lokal; // UB: returnerer referanse til lokal variabel
}
// ANCHOR_END: levetid_dangling_retur

// ANCHOR: levetid_dangling_scope
void dangling_scope() {
    int* p = nullptr;
    {
        int lokal = 42;
        p = &lokal;
    } // lokal destrueres her

    std::cout << "  Verdi: " << *p << std::endl; // UB: lokal finnes ikke lenger
}
// ANCHOR_END: levetid_dangling_scope

// ANCHOR: levetid_vektor_invalidering
void vektor_invalidering() {
    std::vector<int> tall = {1, 2, 3};
    int& ref = tall[0];

    tall.push_back(4); // kan omallokere vektoren

    std::cout << "  ref = " << ref << std::endl; // UB: ref kan være ugyldig
}
// ANCHOR_END: levetid_vektor_invalidering

int main() {
    std::cout << "--- Dangling reference ved retur ---" << std::endl;
    // int& r = hent_verdi(); // UB — dekommentér for å se advarsel
    std::cout << "  (hent_verdi() er ukommentert for å unngå UB ved kjøring)" << std::endl;

    std::cout << "\n--- Dangling peker etter scope ---" << std::endl;
    dangling_scope();

    std::cout << "\n--- Vektor-invalidering ---" << std::endl;
    vektor_invalidering();

    return 0;
}
