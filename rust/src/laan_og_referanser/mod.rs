pub fn main() {
    println!("\n=== Lån og referanser i Rust ===");

    laan_grunnleggende();
}

fn laan_grunnleggende() {
    println!("\n--- Lån og referanser ---");
    // ANCHOR: laan_grunnleggende
    let mut tekst = String::from("hei");

    // Uforanderlig lån — vi kan ha flere samtidig
    let r1 = &tekst;
    let r2 = &tekst;
    println!("  r1 = {r1}, r2 = {r2}");

    // Muterbart lån — kun ett om gangen, og ingen uforanderlige lån aktive
    let r3 = &mut tekst;
    r3.push_str(" verden");
    println!("  r3 = {r3}");
    // ANCHOR_END: laan_grunnleggende
}
