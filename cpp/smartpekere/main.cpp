#include <iostream>
#include <memory>
#include <string>

// ANCHOR: lekkasje_early_return
void kan_lekke(bool tidlig_retur) {
    int* p = new int(42);
    if (tidlig_retur) {
        return; // Minnelekkasje! delete p blir aldri kalt.
    }
    std::cout << *p << std::endl;
    delete p;
}
// ANCHOR_END: lekkasje_early_return

// ANCHOR: unique_ptr_grunnleggende
void unique_ptr_grunnleggende() {
    std::unique_ptr<int> p = std::make_unique<int>(42);
    std::cout << "Verdi: " << *p << std::endl;
    // p frigjøres automatisk her — ingen delete nødvendig
}
// ANCHOR_END: unique_ptr_grunnleggende

// ANCHOR: unique_ptr_move
void unique_ptr_move() {
    std::unique_ptr<std::string> a = std::make_unique<std::string>("hei");
    std::cout << "a: " << *a << std::endl;

    std::unique_ptr<std::string> b = std::move(a); // Eierskap flyttes til b
    // a er nå nullptr
    std::cout << "b: " << *b << std::endl;

    if (!a) {
        std::cout << "a er nullptr etter move" << std::endl;
    }
}
// ANCHOR_END: unique_ptr_move

// ANCHOR: shared_ptr_grunnleggende
void shared_ptr_grunnleggende() {
    std::shared_ptr<std::string> a = std::make_shared<std::string>("delt");
    std::cout << "a: " << *a << " (antall: " << a.use_count() << ")" << std::endl;

    {
        std::shared_ptr<std::string> b = a; // Referansetelling øker
        std::cout << "Inne i blokk — antall: " << a.use_count() << std::endl;
    } // b går ut av scope, referansetelling synker

    std::cout << "Etter blokk — antall: " << a.use_count() << std::endl;
}
// ANCHOR_END: shared_ptr_grunnleggende

// ANCHOR: weak_ptr_syklus
struct Node {
    std::string navn;
    std::shared_ptr<Node> neste;    // Sterk referanse
    std::weak_ptr<Node> forrige;    // Svak referanse — bryter syklus

    Node(std::string n) : navn(std::move(n)) {}
    ~Node() { std::cout << "  ~Node(" << navn << ")" << std::endl; }
};

void weak_ptr_syklus() {
    auto a = std::make_shared<Node>("A");
    auto b = std::make_shared<Node>("B");

    a->neste = b;       // A → B (sterk)
    b->forrige = a;     // B → A (svak) — ingen syklus

    std::cout << "a sin referansetelling: " << a.use_count() << std::endl;
    std::cout << "b sin referansetelling: " << b.use_count() << std::endl;

    // Hente verdien fra en weak_ptr:
    if (auto låst = b->forrige.lock()) {
        std::cout << "b->forrige peker på: " << låst->navn << std::endl;
    }
}
// ANCHOR_END: weak_ptr_syklus

// ANCHOR: make_unique_eksempel
void make_unique_vs_new() {
    // Foretrekk make_unique/make_shared:
    auto god = std::make_unique<int>(42);

    // Unngå dette — mulig minnelekkasje ved unntak:
    std::unique_ptr<int> darlig(new int(42));

    std::cout << "god: " << *god << ", darlig: " << *darlig << std::endl;
}
// ANCHOR_END: make_unique_eksempel

int main() {
    std::cout << "--- Minnelekkasje ved early return ---" << std::endl;
    kan_lekke(true);

    std::cout << "\n--- unique_ptr grunnleggende ---" << std::endl;
    unique_ptr_grunnleggende();

    std::cout << "\n--- unique_ptr move ---" << std::endl;
    unique_ptr_move();

    std::cout << "\n--- shared_ptr grunnleggende ---" << std::endl;
    shared_ptr_grunnleggende();

    std::cout << "\n--- weak_ptr syklus ---" << std::endl;
    weak_ptr_syklus();

    std::cout << "\n--- make_unique vs new ---" << std::endl;
    make_unique_vs_new();

    return 0;
}
