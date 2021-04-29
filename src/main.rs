mod ut;
use ut::*;

fn main() {
    for file in ["ba", "cob_bam"].iter() {
        let mut cursor = FileCursor::from(format!("./data/{}.mp4", file));
        match atomize(&mut cursor) {
            Some(atoms) => {
                println!("\n\n---------- ATOM LIST of {} ----------\n{:#?}\n\n", file, atoms)
            },
            None => panic!("Could not parse atoms of {}", file)
        }
    }
}
