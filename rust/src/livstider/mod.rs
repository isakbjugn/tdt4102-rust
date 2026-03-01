pub fn main() {
    println!("\n=== Livstider i Rust ===");

    laan_grunnleggende();
    livstidsannotasjon();
    livstidselisjon();
    struct_med_livstid();
    static_livstid();
}

fn laan_grunnleggende() {
    println!("\n--- Lån og referanser ---");
    // ANCHOR: livstid_laan_grunnleggende
    let mut tekst = String::from("hei");

    // Uforanderlig lån — vi kan ha flere samtidig
    let r1 = &tekst;
    let r2 = &tekst;
    println!("  r1 = {r1}, r2 = {r2}");

    // Muterbart lån — kun ett om gangen, og ingen uforanderlige lån aktive
    let r3 = &mut tekst;
    r3.push_str(" verden");
    println!("  r3 = {r3}");
    // ANCHOR_END: livstid_laan_grunnleggende
}

fn livstidsannotasjon() {
    println!("\n--- Livstidsannotasjoner ---");
    // ANCHOR: livstid_annotasjon
    fn lengste<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() >= s2.len() {
            s1
        } else {
            s2
        }
    }

    let resultat;
    let tekst1 = String::from("lang streng");
    {
        let tekst2 = String::from("kort");
        resultat = lengste(&tekst1, &tekst2);
        println!("  Lengste: {resultat}");
    }
    // ANCHOR_END: livstid_annotasjon
}

fn livstidselisjon() {
    println!("\n--- Livstidselisjon ---");
    // ANCHOR: livstid_elisjon
    // Kompilatoren utleder livstiden automatisk her:
    // fn forste_ord<'a>(tekst: &'a str) -> &'a str
    fn forste_ord(tekst: &str) -> &str {
        match tekst.find(' ') {
            Some(pos) => &tekst[..pos],
            None => tekst,
        }
    }

    let setning = String::from("Rust er gøy");
    let ord = forste_ord(&setning);
    println!("  Første ord: {ord}");
    // ANCHOR_END: livstid_elisjon
}

// ANCHOR: livstid_struct_type
struct Utdrag<'a> {
    tekst: &'a str,
}

impl<'a> Utdrag<'a> {
    fn ny(tekst: &'a str) -> Utdrag<'a> {
        Utdrag { tekst }
    }

    fn vis(&self) {
        println!("  Utdrag: «{}»", self.tekst);
    }
}
// ANCHOR_END: livstid_struct_type

fn struct_med_livstid() {
    println!("\n--- Struct med livstid ---");
    // ANCHOR: livstid_struct_bruk
    let roman = String::from("Det var en mørk og stormfull natt.");
    let utdrag = Utdrag::ny(&roman[..28]);
    utdrag.vis();
    // ANCHOR_END: livstid_struct_bruk
}

fn static_livstid() {
    println!("\n--- 'static-livstiden ---");
    // ANCHOR: livstid_static
    // Streng-literaler har alltid livstiden 'static
    let s: &'static str = "Jeg lever like lenge som programmet";
    println!("  {s}");

    // Typer uten referanser oppfyller også 'static
    fn skriv_ut<T: std::fmt::Display + 'static>(verdi: T) {
        println!("  {verdi}");
    }
    skriv_ut(42);
    skriv_ut(String::from("eid streng"));
    // ANCHOR_END: livstid_static
}
