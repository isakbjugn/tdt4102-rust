#include <iostream>

int main() {
    // Eksempel 1: Enkel new/delete
    int* p = new int(42);
    std::cout << "Verdien av p er: " << *p << std::endl;

    delete p;

    std::cout << "Verdien av p er: " << *p << std::endl;

    // Eksempel 2: Arrays med new[]/delete[]

    // Eksempel 3: Minnelekkasje ved early return

    return 0;
}
