mod null_sikkerhet;
mod monstermatching;
mod minnehandtering;
mod laan_og_referanser;
mod smartpekere;
mod raii;
mod levetider;

fn main() {
    null_sikkerhet::main();
    monstermatching::main();
    minnehandtering::main();
    laan_og_referanser::main();
    smartpekere::main();
    raii::main();
    levetider::main();
}
