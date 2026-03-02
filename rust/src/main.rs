mod null_sikkerhet;
mod minnehandtering;
mod smartpekere;
mod raii;
mod levetider;

fn main() {
    null_sikkerhet::main();
    minnehandtering::main();
    smartpekere::main();
    raii::main();
    levetider::main();
}
