use std::f64::consts::PI;

// ANCHOR: figur_type
enum Figur {
    Sirkel(f64),
    Rektangel(f64, f64),
    Trekant(f64, f64, f64),
}
// ANCHOR_END: figur_type

pub fn main() {
    figur_match();
    destrukturering_tuppel();
    match_vakter();
    let_else_eksempel();
    while_let_eksempel();
    matches_makro();
}

fn figur_match() {
    // ANCHOR: figur_match
    let figurer = vec![
        Figur::Sirkel(5.0),
        Figur::Rektangel(4.0, 6.0),
        Figur::Trekant(3.0, 4.0, 5.0),
    ];

    for figur in &figurer {
        let areal = match figur {
            Figur::Sirkel(radius) => PI * radius * radius,
            Figur::Rektangel(bredde, hoyde) => bredde * hoyde,
            Figur::Trekant(a, b, c) => {
                // Herons formel
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        };
        println!("Areal: {areal:.2}");
    }
    // ANCHOR_END: figur_match
}

fn destrukturering_tuppel() {
    // ANCHOR: destrukturering_tuppel
    // Destrukturering av tupler i let-bindinger
    let punkt = (3.0, 4.0);
    let (x, y) = punkt;
    println!("x={x}, y={y}");

    // Nestede monstre i match
    let punkter = vec![(0, 0), (1, 0), (0, 1), (3, 4)];
    for punkt in &punkter {
        match punkt {
            (0, 0) => println!("Origo"),
            (x, 0) => println!("Pa x-aksen: x={x}"),
            (0, y) => println!("Pa y-aksen: y={y}"),
            (x, y) => println!("Punkt ({x}, {y})"),
        }
    }
    // ANCHOR_END: destrukturering_tuppel
}

fn match_vakter() {
    // ANCHOR: match_vakter
    let tall = 15;

    let beskrivelse = match tall {
        n if n < 0 => "negativt",
        0 => "null",
        n if n % 2 == 0 => "positivt og partall",
        n if n % 2 != 0 => "positivt og oddetall",
        _ => unreachable!(),
    };
    println!("{tall} er {beskrivelse}");

    // Sjekker er nyttige for a kombinere monster med ekstra betingelser
    let verdi: Option<i32> = Some(-5);
    match verdi {
        Some(n) if n > 0 => println!("Positivt tall: {n}"),
        Some(n) => println!("Ikke-positivt tall: {n}"),
        None => println!("Ingen verdi"),
    }
    // ANCHOR_END: match_vakter
}

fn let_else_eksempel() {
    // ANCHOR: let_else
    fn behandle(input: &str) {
        // let-else: destrukturer eller returner tidlig
        let Ok(tall) = input.parse::<i32>() else {
            println!("'{input}' er ikke et gyldig tall");
            return;
        };

        // Her er `tall` en vanlig i32 - ikke innpakket i Result
        println!("{input} * 2 = {}", tall * 2);
    }

    behandle("21");
    behandle("abc");
    // ANCHOR_END: let_else
}

fn while_let_eksempel() {
    // ANCHOR: while_let
    let mut stabel = vec![1, 2, 3];

    // while let fortsetter sa lenge monsteret matcher
    while let Some(topp) = stabel.pop() {
        println!("Tok av: {topp}");
    }
    println!("Stabelen er tom!");
    // ANCHOR_END: while_let
}

fn matches_makro() {
    // ANCHOR: matches_makro
    let tall = vec![1, -3, 5, -2, 0, 8, -1];

    // matches! returnerer true/false basert pa om verdien matcher monsteret
    let antall_positive: usize = tall.iter().filter(|n| matches!(n, 1..)).count();
    println!("Antall positive: {antall_positive}"); // 3

    // Nyttig for raske sjekker uten full match
    let figur = Figur::Sirkel(5.0);
    let er_sirkel = matches!(figur, Figur::Sirkel(_));
    println!("Er sirkel: {er_sirkel}"); // true
    // ANCHOR_END: matches_makro
}
