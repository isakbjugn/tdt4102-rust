mod null_sikkerhet;
mod monstermatching;
mod minnehandtering;
mod smartpekere;
mod raii;
mod levetider;

fn main() {
    null_sikkerhet::main();
    monstermatching::main();
    minnehandtering::main();
    smartpekere::main();
    raii::main();
    levetider::main();
}
