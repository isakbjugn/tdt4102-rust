use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::thread;

pub fn main() {
    println!("\n=== Smartpekere i Rust ===");

    box_grunnleggende();
    box_rekursiv();
    rc_grunnleggende();
    rc_counting();
    weak_syklus();
    arc_grunnleggende();
    refcell_grunnleggende();
}

fn box_grunnleggende() {
    // ANCHOR: box_grunnleggende
    let b = Box::new(42);
    println!("Verdien i boksen: {b}");
    // b frigjøres automatisk her
    // ANCHOR_END: box_grunnleggende
}

// ANCHOR: box_rekursiv_type
enum Liste {
    Element(i32, Box<Liste>),
    Slutt,
}
// ANCHOR_END: box_rekursiv_type

fn box_rekursiv() {
    // ANCHOR: box_rekursiv
    let liste = Liste::Element(1,
        Box::new(Liste::Element(2,
            Box::new(Liste::Element(3,
                Box::new(Liste::Slutt))))));

    // Skriv ut listen
    let mut gjeldende = &liste;
    while let Liste::Element(verdi, neste) = gjeldende {
        print!("{verdi} → ");
        gjeldende = neste;
    }
    println!("slutt");
    // ANCHOR_END: box_rekursiv
}

fn rc_grunnleggende() {
    // ANCHOR: rc_grunnleggende
    let a = Rc::new(String::from("delt verdi"));

    let b = Rc::clone(&a); // Øker referansetelleren
    let c = Rc::clone(&a);

    println!("a: {a}, b: {b}, c: {c}");
    // ANCHOR_END: rc_grunnleggende
}

fn rc_counting() {
    // ANCHOR: rc_counting
    let a = Rc::new(String::from("hei"));
    println!("Etter opprettelse:   {}", Rc::strong_count(&a));

    {
        let _b = Rc::clone(&a);
        println!("Inne i blokk:        {}", Rc::strong_count(&a));
    } // _b går ut av scope

    println!("Etter blokk:         {}", Rc::strong_count(&a));
    // ANCHOR_END: rc_counting
}

// ANCHOR: weak_syklus_type
#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    navn: String,
    neste: Option<Rc<Node>>,
    forrige: Option<Weak<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("  ~Node({})", self.navn);
    }
}
// ANCHOR_END: weak_syklus_type

fn weak_syklus() {
    // ANCHOR: weak_syklus
    let a = Rc::new(Node {
        navn: String::from("A"),
        neste: None,
        forrige: None,
    });

    let b = Rc::new(Node {
        navn: String::from("B"),
        neste: None,
        forrige: Some(Rc::downgrade(&a)), // Svak referanse til A
    });

    println!("a sterke ref: {}", Rc::strong_count(&a));
    println!("a svake ref:  {}", Rc::weak_count(&a));

    // Hente verdien fra en Weak-referanse:
    if let Some(forrige) = &b.forrige {
        if let Some(node) = forrige.upgrade() {
            println!("b.forrige peker på: {}", node.navn);
        }
    }
    // ANCHOR_END: weak_syklus
}

fn arc_grunnleggende() {
    // ANCHOR: arc_grunnleggende
    let data = Arc::new(vec![1, 2, 3]);

    let data_klon = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("Fra tråd: {data_klon:?}");
    });

    println!("Fra main:  {data:?}");
    handle.join().unwrap();
    // ANCHOR_END: arc_grunnleggende
}

fn refcell_grunnleggende() {
    // ANCHOR: refcell_grunnleggende
    let data = RefCell::new(vec![1, 2, 3]);

    // Kan mutere gjennom en uforanderlig binding:
    data.borrow_mut().push(4);

    println!("Etter push: {:?}", data.borrow());

    // Flere uforanderlige lån samtidig er OK:
    let laan1 = data.borrow();
    let laan2 = data.borrow();
    println!("laan1: {laan1:?}, laan2: {laan2:?}");
    // ANCHOR_END: refcell_grunnleggende
}
