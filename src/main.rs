use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::path::Path;

fn main() {
    let path = Path::new("./ba.mp4");

    let mut file = match File::open(&path) {
        Err(err) => panic!("Could not open {}: {}", path.display(), err),
        Ok(file) => file
    };

    let asize = read_atom_chunk(&mut file);
    println!("Atom size: {} ({:X?})", u32::from_be_bytes(asize), asize);

    let atype = read_atom_chunk(&mut file);
    println!("Atom type: {} ({:X?})", str::from_utf8(&atype).unwrap(), atype);

    let astyp = read_atom_chunk(&mut file);
    println!("Atom sub-type: {} ({:X?})", str::from_utf8(&astyp).unwrap(), astyp);
    
    let chunk = consume_chunk(&mut file, (u32::from_be_bytes(asize) - 4*3 as u32) as usize);
    println!("Consumed chunk: {:X?}", chunk);


    for i in 0..1 {
        let asize = read_atom_chunk(&mut file);
        println!("Atom[{}] size: {} ({:X?})", i, u32::from_be_bytes(asize), &asize);

        let atype = read_atom_chunk(&mut file);
        println!("Atom[{}] type: {} ({:X?})", i, str::from_utf8(&atype).unwrap(), &atype);

        let csize = (u32::from_be_bytes(asize) - 4*3 as u32) as usize;
        let chunk = consume_chunk(&mut file, csize);
        println!("Consumed chunk of size {}", csize);
    }

    let atom = Atom::from(&mut file);
}

struct Atom
{
    chunk: Vec<u8>,
    size: u32,
    kind: String,
    atoms: Option<Vec<Atom>>
}

impl Atom {
    fn from(file: &mut File) -> Atom {
        let size = read_atom_chunk(&mut file);
        let size = (4*3 as u32 - u32::from_be_bytes(size)) as usize;
        
        let kind = read_atom_chunk(&mut file);

        let chunk = consume_chunk(&mut file, size);

        Atom {
            size: (size as u32),
            kind: str::from_utf8(&kind).unwrap().to_string(),
            chunk,
            atoms: None
        }
    }
}


fn read_atom_chunk(file: &mut File) -> [u8; 4] {
    let mut out: [u8; 4] = [0, 0, 0, 0];
    if file.read(&mut out).unwrap() == 4 {
        out
    } else {
        panic!("Could not read next atom chunk")
    }
}

fn consume_chunk(file: &mut File, remainder: usize) -> Vec<u8> {
    let mut v = vec![0 as u8; remainder];
    if file.read(&mut v).unwrap() == remainder {
        v
    } else {
        panic!("Could not consume the chunk (asked for {})", remainder)
    }
}