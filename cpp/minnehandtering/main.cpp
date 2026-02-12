#include <iostream>

int main() {
    // ANCHOR: enkel_allokering
    int* p = new int(42);
    std::cout << "Verdien av p er: " << *p << std::endl;
    delete p;
    // ANCHOR_END: enkel_allokering

    // ANCHOR: use_after_free
    std::cout << "Verdien av p er: " << *p << std::endl; // UB!
    // ANCHOR_END: use_after_free

    // Eksempel 2: Arrays med new[]/delete[]

    // Eksempel 3: Minnelekkasje ved early return

    return 0;
}
