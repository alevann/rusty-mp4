mod ut;
use ut::*;

fn main() {
    let mut cursor = FileCursor::from(format!("./data/ba.mp4"));
    atomize(&mut cursor);
}
