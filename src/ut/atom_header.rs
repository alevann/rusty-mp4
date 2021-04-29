use super::file_cursor::FileCursor;
use std::str;

pub fn atomize(cursor: &mut FileCursor) {
    if cursor.consumed() {
        return
    }

    match AtomHeader::from_cursor(cursor) {
        Some(hdr) => {
            print!("Found a header starting at {}: {:#?}\nChecking next 8 bytes... ", hdr.pos, hdr);
            atomize(cursor);
            println!("Moving to check for a header starting at {}", hdr.off);
            cursor.move_to(hdr.off + hdr.pos);
            atomize(cursor);
        },
        None => println!("No header found at {}", cursor.offset),
    }
}

#[derive(Debug)]
pub struct AtomHeader
{
    off: usize,
    pos: usize,
    sig: String,
}

impl AtomHeader {
    fn from_cursor(cursor: &mut FileCursor) -> Option<AtomHeader> {
        let pos = cursor.offset;
        let off = offset(cursor);
        match signature(cursor) {
            Some(sig) => Some(AtomHeader { sig, off, pos }),
            None => None
        }
    }
}

// Allowed atom headers signatures
const VSGI: [&str; 171] = [ "ainf", "avcn", "bloc", "bpcc", "buff", "bxml", "ccid", "cdef", "clip", "cmap", "co64", "coin", "colr", "crgn", "crhd", "cslg", "ctab", "ctts", "cvru", "dinf", "dref", "dsgd", "dstg", "edts", "elst", "emsg", "evti", "fdel", "feci", "fecr", "fiin", "fire", "fpar", "free", "frma", "ftyp", "gitn", "grpi", "hdlr", "hmhd", "hpix", "icnu", "ID32", "idat", "ihdr", "iinf", "iloc", "imap", "imif", "infe", "infu", "iods", "iphd", "ipmc", "ipro", "iref", "jP$20$20", "jp2c", "jp2h", "jp2i", "kmat", "leva", "load", "loop", "lrcu", "m7hd", "matt", "mdat", "mdhd", "mdia", "mdri", "meco", "mehd", "mere", "meta", "mfhd", "mfra", "mfro", "minf", "mjhd", "moof", "moov", "mvcg", "mvci", "mvex", "mvhd", "mvra", "nmhd", "ochd", "odaf", "odda", "odhd", "odhe", "odrb", "odrm", "odtt", "ohdr", "padb", "paen", "pclr", "pdin", "pitm", "pnot", "prft", "pssh", "res$20", "resc", "resd", "rinf", "saio", "saiz", "sbgp", "schi", "schm", "sdep", "sdhd", "sdtp", "sdvp", "segr", "senc", "sgpd", "sidx", "sinf", "skip", "smhd", "srmb", "srmc", "srpp", "ssix", "stbl", "stco", "stdp", "sthd", "strd", "stri", "stsc", "stsd", "stsg", "stsh", "stss", "stsz", "stts", "styp", "stz2", "subs", "swtc", "tfad", "tfdt", "tfhd", "tfma", "tfra", "tibr", "tiri", "tkhd", "traf", "trak", "tref", "trex", "trgr", "trik", "trun", "udta", "uinf", "UITS", "ulst", "url$20", "uuid", "vmhd", "vwdi", "xml$20", "xml$20" ];

fn valid(sig: &String) -> bool {
    VSGI.iter().any(|i| sig == i)
}

fn signature(cursor: &mut FileCursor) -> Option<String> {
    let sig = cursor.read_exact::<4>();
    match str::from_utf8(&sig) {
        Ok(value) => valid_sig(String::from(value)),
        Err(_) => None
    }
}

fn valid_sig(sig: String) -> Option<String> {
    if valid(&sig) {
        Some(sig)
    } else {
        None
    }
}

fn offset(cursor: &mut FileCursor) -> usize {
    let off = cursor.read_exact::<4>();
    u32::from_be_bytes(off) as usize
}
