use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;

struct FileCursor
{
    offset: usize,
    data: Vec<u8>,
}

impl FileCursor {
    fn from(path: String) -> FileCursor {
        let data = fs::read(path).expect("Unable to read file");
        FileCursor {
            offset: 0,
            data
        }
    }

    // NOTE: currently returns a COPY of the content read,
    // this can be a lot of memory occupied if the content is large.
    fn read(self: &mut FileCursor, count: usize) -> Vec<u8> {
        self.offset += count;
        self.peek(self.offset - count)
    }

    // NOTE: currently returns a COPY of the content read,
    // this can be a lot of memory occupied if the content is large.
    fn peek(self: &FileCursor, count: usize) -> Vec<u8> {
        Vec::from_iter(self.data[self.offset..count].iter().cloned())
    }

    fn skip(self: &mut FileCursor, count: usize) {
        self.offset += count;
    }
}

fn fmain() {
    let mut cursor = FileCursor::from(String::from("./ba.mp4"));
    let size = cursor.read(4);
}

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

    let _ = read_atom_chunk(&mut file);
    let atom = Atom::from(&mut file);

    fmain();
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
        let size = read_atom_chunk(file);
        let kind = read_atom_chunk(file);
        let size = (u32::from_be_bytes(size) - 4*3 as u32) as usize;
        let chunk = consume_chunk(file, size);



        Atom {
            size: (size as u32),
            kind: String::from(str::from_utf8(&kind).unwrap()),
            chunk,
            atoms: None
        }
    }
}

fn peek_atom_chunk(file: &mut File) -> [u8; 4] {
    let mut out: [u8; 4] = [0, 0, 0, 0];
    if file.read(&mut out).unwrap() == 4 {
        out
    } else {
        panic!("Could not read next atom chunk")
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
