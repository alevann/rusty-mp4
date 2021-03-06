use super::{file_cursor::FileCursor, node::Node};
use std::str;


pub fn atomize(cursor: &mut FileCursor, parent: Option<&Node<AtomHeader>>) -> Option<Vec<Node<AtomHeader>>> {
    if cursor.consumed() {
        println!("EOF");
        return None
    }

    let mof = if let Some(parent) = parent {
        if let Some(data) = &parent.data {
            data.off
        } else {
            cursor.count
        }
    } else {
        cursor.count
    };

    println!("Looking for a header at {}", cursor.offset);
    match AtomHeader::from_cursor(cursor) {
        None => {
            println!("Could not find a valid header at {}", cursor.offset-8);
            None
        },
        Some(hdr) => {
            if hdr.pos >= mof {
                println!("Header {} at {} is >= than {} which is the max value", hdr.sig, hdr.pos, mof);
                return None
            } else {
                println!("Inline {} where max {}", hdr.pos, mof);
            }

            println!("Found a valid header at {}", hdr.pos);
            println!("Looking for neighboor nodes of {} (at {})...", hdr.sig, hdr.off);
            cursor.move_to(hdr.pos+hdr.off);
            let ngh = atomize(cursor, parent);
            
            if let Some(_) = ngh {
                println!("Found a neighboor of {} at {}", hdr.sig, hdr.off);
            } else {
                println!("No neighboors found at {} ({})", hdr.off, hdr.sig);
            }

            let mut nodes = Node::from(hdr.clone());

            println!("Starting to look for children of {} (from {} up to {})", hdr.sig, hdr.pos, hdr.pos+hdr.off);
            cursor.move_to(hdr.pos+8);
            let chd = atomize(cursor, Some(&nodes));

            let mut khd: Option<Vec<Node<AtomHeader>>> = None;
            if let Some(chd) = chd {
                println!("Found children/s of {} (count: {})", hdr.sig, chd.len());
                khd = Some(chd);
            } else {
                println!("No neighboors found at {} ({})", hdr.off, hdr.sig);
            }

            if let Some(chd) = khd {
                for node in chd {
                    nodes.add(node);
                }
            }

            let mut nodes = vec![nodes];
            if let Some(mut ngh) = ngh {
                nodes.append(&mut ngh);
            }
            
            Some(nodes)
        }
    }
}

pub fn _atomize(cursor: &mut FileCursor) -> Option<Vec<Node<AtomHeader>>> {
    if cursor.consumed() {
        return None
    }

    match AtomHeader::from_cursor(cursor) {
        Some(hdr) => {
            print!("Found a header starting at {}: {:#?}\nChecking next 8 bytes... ", hdr.pos, hdr);
            let apc = _atomize(cursor);

            println!("Moving to check for a header starting at {}", hdr.off);
            cursor.move_to(hdr.off + hdr.pos);
            let apn = _atomize(cursor);

            let mut nodes = Node::from(hdr);
            if let Some(apc) = apc {
                for node in apc {
                    nodes.add(node);
                }
            }

            let mut nodes = vec![nodes];
            if let Some(mut apn) = apn {
                nodes.append(&mut apn);
            }
            
            Some(nodes)
        },
        None => {
            println!("No header found at {}", cursor.offset);
            None
        },
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct AtomHeader
{
    pub off: usize,
    pub pos: usize,
    pub sig: String,
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
const VSGI: [&str; 171] = [ "ainf", "avcn", "bloc", "bpcc", "buff", "bxml", "ccid", "cdef", "clip", "cmap", "co64", "coin", "colr", "crgn", "crhd", "cslg", "ctab", "ctts", "cvru", "dinf", "dref", "dsgd", "dstg", "edts", "elst", "emsg", "evti", "fdel", "feci", "fecr", "fiin", "fire", "fpar", "free", "frma", "ftyp", "gitn", "grpi", "hdlr", "hmhd", "hpix", "icnu", "ID32", "idat", "ihdr", "iinf", "iloc", "imap", "imif", "infe", "infu", "iods", "iphd", "ipmc", "ipro", "iref", "jP$20$20", "jp2c", "jp2h", "jp2i", "kmat", "leva", "load", "loop", "lrcu", "m7hd", "matt", "mdat", "mdhd", "mdia", "mdri", "meco", "mehd", "mere", "meta", "mfhd", "mfra", "mfro", "minf", "mjhd", "moof", "moov", "mvcg", "mvci", "mvex", "mvhd", "mvra", "nmhd", "ochd", "odaf", "odda", "odhd", "odhe", "odrb", "odrm", "odtt", "ohdr", "padb", "paen", "pclr", "pdin", "pitm", "pnot", "prft", "pssh", "res$20", "resc", "resd", "rinf", "saio", "saiz", "sbgp", "schi", "schm", "sdep", "sdhd", "sdtp", "sdvp", "segr", "senc", "sgpd", "sidx", "sinf", "skip", "smhd", "srmb", "srmc", "srpp", "ssix", "stbl", "stco", "stdp", "sthd", "strd", "stri", "stsc", "stsd", "stsg", "stsh", "stss", "stsz", "stts", "styp", "stz2", "subs", "swtc", "tfad", "tfdt", "tfhd", "tfma", "tfra", "tibr", "tiri", "tkhd", "traf", "trak", "tref", "trex", "trgr", "trik", "trun", "udta", "uinf", "UITS", "ulst", "url", "uuid", "vmhd", "vwdi", "xml$20", "xml" ];

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
