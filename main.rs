mod ut;
use ut::*;

fn main() {
    let mut cursor = FileCursor::from(format!("./data/ba.mp4"));
    match atomize(&mut cursor) {
        Some(atoms) => {
            print!("---------- ATOM LIST ----------\n{:#?}", atoms)
        },
        None => panic!("Could not parse atoms!")
    }
}
