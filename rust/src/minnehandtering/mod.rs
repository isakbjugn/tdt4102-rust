pub fn hello() {
    println!("Kvekk fra rust!");

    let a = 4;
    println!("a har verdien {}", a);
    let b = a;
    println!("b har verdien {}", b);
    println!("a har verdien {}", a);

    let c = 10;
    println!("a har verdien {}", c);
    function_that_copies(c);
    println!("a har verdien {}", c);

    let a_string = String::from("Dette var en fin streng!");
    println!("{a_string}");
    let b_string = a_string;
    println!("{b_string}");
    //println!("{a_string}");

    let c_string = String::from("Nok en fin streng!");
    function_that_moves(c_string);
    // println!("{c_string}"); // Kompileringsfeil! Verdien er flyttet.
}

fn function_that_copies(value: i32) {
    println!("Den innsendte variablen har verdien {}", value);
}

fn function_that_moves(string: String) {
    println!("Den innsendte variablen har verdien {}", string);
}
