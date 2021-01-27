use std::{env, fs};
use rsbrainfxxk::interpreter::Interpreter as Engine;

fn main() {
    let file = env::args().skip(1).next().expect("No file specfied");
    let source = fs::read_to_string(&file).unwrap();
    Engine::new().from_source(&source).unwrap();
}
