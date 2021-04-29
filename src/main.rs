use std::fs;
use std::iter::FromIterator;
use std::fmt;
use std::str;

struct FileCursor
{
    offset: usize,
    data: Vec<u8>,
    count: usize
}

impl FileCursor {
    fn from(path: String) -> FileCursor {
        let data = fs::read(path).expect("Unable to read file");
        FileCursor {
            offset: 0,
            count: data.len(),
            data
        }
    }

    fn read_exact<const S: usize>(&mut self) -> [u8; S] {
        let mut out = [0 as u8; S];
        for (i, v) in self.data[self.offset..self.offset+S]
                            .iter().cloned().enumerate() {
            out[i] = v;
        }
        self.offset += S;
        out
    }

    fn read(&mut self, count: usize) -> Vec<u8> {
        let out = Vec::from_iter(
            self.data[self.offset..self.offset+count].iter().cloned());
        self.offset += count;
        out
    }

    fn skip(&mut self, count: usize) -> usize {
        self.offset += count;
        self.offset
    }
}

struct Atom
{
    offset: usize,
    size: usize,
    kind: String,
    chunk: Vec<u8>
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Atom")
         .field("offset", &self.offset)
         .field("size", &self.size)
         .field("kind", &self.kind)
         .field("chunk (len)", &self.chunk.len())
         .finish()
    }

}

impl Atom {
    fn from_cursor(cursor: &mut FileCursor) -> Atom {
        let offset = cursor.offset;

        let size = cursor.read_exact::<4>();
        println!("Atom size: {} ({:X?})", u32::from_be_bytes(size), size);

        let kind = cursor.read_exact::<4>();
        println!("Atom kind: {} ({:X?})", str::from_utf8(&kind).unwrap(), kind);

        let size = u32::from_be_bytes(size) as usize - 4*2;
        let chunk = cursor.read(size);
        println!("Atom chunk size: {} (now at offset {} out of {} total)\n", size, cursor.offset, cursor.count);
        Atom {
            offset,
            size,
            kind: String::from(str::from_utf8(&kind).unwrap()),
            chunk
        }
    }

    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Atom {
        let mut size: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            size[i] = buffer[offset+i];
        }
        let mut kind: [u8; 4] = [0, 0, 0, 0];
        for i in 4..8 {
            kind[i-4] = buffer[offset+i];
        }
        let size = u32::from_be_bytes(size) as usize;
        let chunk = &buffer[8..size - 4*2];

        Atom {
            offset,
            size,
            kind: String::from(str::from_utf8(&kind).unwrap()),
            chunk: chunk.iter().cloned().collect()
        }
    }

    fn parse_subatoms(&self) -> Option<Vec<Atom>> {
        match self.kind.as_str() {
            "ftyp" => { println!("ftyp atom is not supported yet"); None },
            "moov" => self.parse_moov(),
            "mdat" => { println!("mdat atom is not supported yet"); None },
            "mvhd" => { println!("mdat atom is not supported yet"); None },
            _ => panic!("unknown atom: {}", self.kind)
        }
    }

    fn parse_moov(&self) -> Option<Vec<Atom>> {
        if self.chunk.is_empty() {
            None
        } else {
            let mut atoms: Vec<Atom> = Vec::new();
            let mut offset = 0 as usize;

            while offset < self.size {
                println!("offset: {} - size: {}", offset, self.size);
                let atom = Atom::from_buffer(&self.chunk, offset);
                offset += atom.size;
                atoms.push(atom);
            }

            Some(atoms)
        }
    }
}

fn parse_file(file: String) {
    let mut cursor = FileCursor::from(format!("./data/{}.mp4", file));
    println!("Parsing file: {}", file);
    for i in 1..4 {
        println!("--- Atom {} ---", i);
        
        let atom = Atom::from_cursor(&mut cursor);
        println!("{:#?}", atom);

        for atom in atom.parse_subatoms() {
            println!("{:#?}", atom);
        }
    }
}

fn main() {
    let files = ["ba", "cob_bam"];

    for file in files.iter() {
        parse_file(file.to_string());
    }
}
