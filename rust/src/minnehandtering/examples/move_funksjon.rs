fn function_that_moves(string: String) {
    println!("Den innsendte variablen har verdien {}", string);
}

fn main() {
    let c_string = String::from("Nok en fin streng!");
    function_that_moves(c_string);
    // println!("{c_string}"); // Kompileringsfeil! Verdien er flyttet.
}
