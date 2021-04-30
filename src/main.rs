mod ut;
use ut::*;

fn main() {
    for file in ["ba", "cob_bam"].iter() {
        let mut cursor = FileCursor::from(format!("./data/{}.mp4", file));
        let count = cursor.count;
        match atomize(&mut cursor, None) {
            Some(atoms) => {
                println!("\n\n---------- ATOM LIST of {} ----------\n{:#?}\n\n", file, atoms);
                print!("[");
                for atom in atoms {
                    atom.apply_recursive(&|atom: &AtomHeader| print!("{}, ", atom.pos));
                }
                print!("]\n\n");
            },
            None => panic!("Could not parse atoms of {}", file)
        }
    }
}
