mod ut;
mod atom;
use ut::*;
use atom::{AtomDescriptor,describe};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let paths = fs::read_dir("./data")?;
    let mut files = Vec::new();
    for entry in paths {
        if let Ok(entry) = entry {
            files.push(entry.path().display().to_string());
        } else {
            println!("Skipping file");
        }
    }

    for file in files {
        let mut cursor = FileCursor::from(file.clone());
        let count = cursor.count;
        if let Some(atoms) = describe(&mut cursor, count) {
            println!("\n\n---------- ATOM LIST of {} ----------\n{:#?}\n\n", file, atoms);
        } else {
            println!("\n\nCould not parse atoms of {}\n\n", file);
        }
    }

    Ok(())
}
