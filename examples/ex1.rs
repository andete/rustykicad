// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

extern crate rustykicad;

fn main() {
    let s = rustykicad::read_module("../examples/SILABS_EFM32_QFN24.kicad_mod").unwrap();
    println!("{}", s);
}
