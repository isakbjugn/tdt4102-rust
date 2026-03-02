use std::collections::HashMap;

pub fn main() {
    option_grunnleggende();
    option_pattern_matching();
    option_metoder();
    result_grunnleggende();
    if let Err(e) = result_sporsmalstegn() {
        println!("Feil: {e}");
    }
    hashmap_eksempel();
}

fn option_grunnleggende() {
    // ANCHOR: option_grunnleggende
    let partall: Option<i32> = Some(42);
    let ingen_verdi: Option<i32> = None;

    println!("partall = {:?}", partall);   // Some(42)
    println!("ingen_verdi = {:?}", ingen_verdi); // None
    // ANCHOR_END: option_grunnleggende
}

fn option_pattern_matching() {
    // ANCHOR: option_pattern_matching
    let tall: Option<i32> = Some(42);

    // match tvinger deg til å håndtere begge tilfeller
    match tall {
        Some(verdi) => println!("Fant verdien: {verdi}"),
        None => println!("Ingen verdi"),
    }

    // if let er kortere når du bare bryr deg om ett tilfelle
    if let Some(verdi) = tall {
        println!("Verdien er {verdi}");
    }
    // ANCHOR_END: option_pattern_matching
}

fn option_metoder() {
    // ANCHOR: option_metoder
    let kanskje_tall: Option<i32> = None;

    // Gi en standardverdi hvis None
    let med_standard = kanskje_tall.unwrap_or(0);
    println!("med_standard = {med_standard}"); // 0

    // Transformer verdien inni Option
    let kanskje_tall: Option<i32> = Some(5);
    let dobbel = kanskje_tall.map(|x| x * 2);
    println!("dobbel = {:?}", dobbel); // Some(10)

    // Kjed operasjoner som selv returnerer Option
    let tekst: Option<&str> = Some("42");
    let parsed = tekst.and_then(|t| t.parse::<i32>().ok());
    println!("parsed = {:?}", parsed); // Some(42)
    // ANCHOR_END: option_metoder
}

fn result_grunnleggende() {
    // ANCHOR: result_grunnleggende
    fn del(teller: i32, nevner: i32) -> Result<i32, String> {
        if nevner == 0 {
            return Err(String::from("Kan ikke dele på null"));
        }
        Ok(teller / nevner)
    }

    match del(10, 2) {
        Ok(resultat) => println!("10 / 2 = {resultat}"),
        Err(feil) => println!("Feil: {feil}"),
    }

    match del(10, 0) {
        Ok(resultat) => println!("10 / 0 = {resultat}"),
        Err(feil) => println!("Feil: {feil}"),
    }
    // ANCHOR_END: result_grunnleggende
}

fn result_sporsmalstegn() -> Result<(), String> {
    // ANCHOR: result_sporsmalstegn
    fn parse_og_doble(tekst: &str) -> Result<i32, String> {
        let tall: i32 = tekst
            .parse()
            .map_err(|_| format!("Kunne ikke parse '{tekst}' som tall"))?;
        Ok(tall * 2)
    }

    println!("{}", parse_og_doble("21")?); // 42
    // parse_og_doble("abc") ville returnert Err
    // ANCHOR_END: result_sporsmalstegn
    Ok(())
}

// Brukt i sammenlikningen — HashMap::get returnerer Option
pub fn hashmap_eksempel() {
    // ANCHOR: hashmap_option
    let karakterer = HashMap::from([
        (String::from("Ola"), 'A'),
        (String::from("Kari"), 'B'),
    ]);

    // HashMap::get returnerer Option<&V> — aldri null
    match karakterer.get("Per") {
        Some(karakter) => println!("Per fikk {karakter}"),
        None => println!("Per er ikke registrert"),
    }
    // ANCHOR_END: hashmap_option
}
