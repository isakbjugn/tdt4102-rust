#include <cstdio>
#include <fstream>
#include <iostream>
#include <mutex>
#include <string>
#include <thread>

// ANCHOR: raii_fil_uten_raii
void skriv_til_fil_manuell(bool feil) {
    FILE* f = fopen("cpp/raii/logg.txt", "w");
    if (!f) return;

    if (feil) {
        return; // Ressurslekkasje! fclose(f) blir aldri kalt.
    }

    fprintf(f, "Alt gikk bra\n");
    fclose(f);
}
// ANCHOR_END: raii_fil_uten_raii

// ANCHOR: raii_fil_med_raii
void skriv_til_fil_raii() {
    std::ofstream fil("cpp/raii/logg.txt");
    fil << "Alt gikk bra" << std::endl;
    // fil lukkes automatisk når den går ut av scope
}
// ANCHOR_END: raii_fil_med_raii

// ANCHOR: raii_custom_loggfil
class LoggFil {
    std::ofstream fil;
    std::string filnavn;

public:
    LoggFil(const std::string& navn) : fil(navn), filnavn(navn) {
        std::cout << "  Åpner loggfil: " << filnavn << std::endl;
        fil << "=== Logg startet ===" << std::endl;
    }

    void skriv(const std::string& melding) {
        fil << melding << std::endl;
    }

    ~LoggFil() {
        fil << "=== Logg avsluttet ===" << std::endl;
        std::cout << "  Lukker loggfil: " << filnavn << std::endl;
        // fil (std::ofstream) lukkes automatisk av sin egen destruktør
    }
};
// ANCHOR_END: raii_custom_loggfil

// ANCHOR: raii_lock_guard
std::mutex mtx;
int delt_teller = 0;

void oek_teller() {
    std::lock_guard<std::mutex> laas(mtx); // Låsen tas her
    ++delt_teller;
    std::cout << "  Teller: " << delt_teller << std::endl;
    // laas går ut av scope — mutex frigjøres automatisk
}
// ANCHOR_END: raii_lock_guard

// ANCHOR: raii_destruktor_rekkefolge
struct Ressurs {
    std::string navn;

    Ressurs(const std::string& n) : navn(n) {
        std::cout << "  Oppretter: " << navn << std::endl;
    }

    ~Ressurs() {
        std::cout << "  Frigjør:   " << navn << std::endl;
    }
};

void destruktor_rekkefolge() {
    Ressurs a("A");
    Ressurs b("B");
    Ressurs c("C");
    // Destruktører kalles i omvendt rekkefølge: C, B, A
}
// ANCHOR_END: raii_destruktor_rekkefolge

int main() {
    std::cout << "--- Fil uten RAII ---" << std::endl;
    skriv_til_fil_manuell(true);

    std::cout << "\n--- Fil med RAII (ofstream) ---" << std::endl;
    skriv_til_fil_raii();

    std::cout << "\n--- Custom RAII: LoggFil ---" << std::endl;
    {
        LoggFil logg("cpp/raii/eksempel.log");
        logg.skriv("Første melding");
        logg.skriv("Andre melding");
    }

    std::cout << "\n--- lock_guard ---" << std::endl;
    std::thread t1(oek_teller);
    std::thread t2(oek_teller);
    t1.join();
    t2.join();

    std::cout << "\n--- Destruktorrekkefølge ---" << std::endl;
    destruktor_rekkefolge();

    return 0;
}
