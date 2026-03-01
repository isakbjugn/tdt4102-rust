use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

pub fn main() {
    println!("\n=== RAII i Rust ===");

    fil_eksempel();
    custom_loggfil();
    mutex_guard();
    drop_rekkefolge();
    tidlig_drop();
}

fn fil_eksempel() {
    // ANCHOR: raii_fil
    let mut fil = File::create("rust/src/raii/logg.txt").unwrap();
    writeln!(fil, "Alt gikk bra").unwrap();
    // fil lukkes automatisk når den går ut av scope —
    // File implementerer Drop
    // ANCHOR_END: raii_fil
}

// ANCHOR: raii_custom_loggfil_type
struct LoggFil {
    fil: File,
    filnavn: String,
}

impl LoggFil {
    fn ny(filnavn: &str) -> LoggFil {
        let mut fil = File::create(filnavn).unwrap();
        println!("  Åpner loggfil: {filnavn}");
        writeln!(fil, "=== Logg startet ===").unwrap();
        LoggFil {
            fil,
            filnavn: filnavn.to_string(),
        }
    }

    fn skriv(&mut self, melding: &str) {
        writeln!(self.fil, "{melding}").unwrap();
    }
}

impl Drop for LoggFil {
    fn drop(&mut self) {
        writeln!(self.fil, "=== Logg avsluttet ===").unwrap();
        println!("  Lukker loggfil: {}", self.filnavn);
        // self.fil droppes automatisk etter dette — filen lukkes
    }
}
// ANCHOR_END: raii_custom_loggfil_type

fn custom_loggfil() {
    // ANCHOR: raii_custom_loggfil
    let mut logg = LoggFil::ny("rust/src/raii/eksempel.log");
    logg.skriv("Første melding");
    logg.skriv("Andre melding");
    // logg droppes her — avslutningslinjen skrives og filen lukkes
    // ANCHOR_END: raii_custom_loggfil
}

fn mutex_guard() {
    // ANCHOR: raii_mutex
    let data = Mutex::new(0);

    {
        let mut guard = data.lock().unwrap(); // Låsen tas her
        *guard += 1;
        println!("  Teller: {guard}");
        // guard går ut av scope — låsen frigjøres automatisk
    }

    // Låsen er nå frigjort — vi kan ta den igjen
    let mut guard = data.lock().unwrap();
    *guard += 1;
    println!("  Teller: {guard}");
    // ANCHOR_END: raii_mutex
}

// ANCHOR: raii_drop_rekkefolge_type
struct Ressurs {
    navn: String,
}

impl Ressurs {
    fn ny(navn: &str) -> Ressurs {
        println!("  Oppretter: {navn}");
        Ressurs {
            navn: navn.to_string(),
        }
    }
}

impl Drop for Ressurs {
    fn drop(&mut self) {
        println!("  Frigjør:   {}", self.navn);
    }
}
// ANCHOR_END: raii_drop_rekkefolge_type

fn drop_rekkefolge() {
    // ANCHOR: raii_drop_rekkefolge
    let _a = Ressurs::ny("A");
    let _b = Ressurs::ny("B");
    let _c = Ressurs::ny("C");
    // Droppes i omvendt rekkefølge: C, B, A
    // ANCHOR_END: raii_drop_rekkefolge
}

fn tidlig_drop() {
    // ANCHOR: raii_tidlig_drop
    let a = Ressurs::ny("X");
    let _b = Ressurs::ny("Y");

    println!("  Før drop");
    drop(a); // Frigjør X eksplisitt — a kan ikke brukes etterpå
    println!("  Etter drop");
    // ANCHOR_END: raii_tidlig_drop
}
