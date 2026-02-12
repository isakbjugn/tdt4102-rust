fn main() {
    // ANCHOR: copy_semantikk
    let a = 4;
    println!("a har verdien {}", a);
    let b = a;
    println!("b har verdien {}", b);
    println!("a har verdien {}", a); // OK - i32 implementerer Copy
    // ANCHOR_END: copy_semantikk

    // ANCHOR: copy_funksjon
    let c = 10;
    println!("c har verdien {}", c);
    function_that_copies(c);
    println!("c har verdien {}", c); // OK - verdien ble kopiert
    // ANCHOR_END: copy_funksjon

    // ANCHOR: move_semantikk
    let a_string = String::from("Dette var en fin streng!");
    println!("{a_string}");
    let b_string = a_string; // a_string er nå flyttet (moved)
    println!("{b_string}");
    // println!("{a_string}"); // Kompileringsfeil! Verdien er flyttet.
    // ANCHOR_END: move_semantikk

    // ANCHOR: move_funksjon
    let c_string = String::from("Nok en fin streng!");
    function_that_moves(c_string);
    // println!("{c_string}"); // Kompileringsfeil! Verdien er flyttet.
    // ANCHOR_END: move_funksjon
}

fn function_that_copies(value: i32) {
    println!("Den innsendte variablen har verdien {}", value);
}

fn function_that_moves(string: String) {
    println!("Den innsendte variablen har verdien {}", string);
}
