use crate::ut::{FileCursor,Node};
use std::str;

pub fn describe(cursor: &mut FileCursor, mof: usize) -> Option<Vec<Node<AtomDescriptor>>> {
    if cursor.consumed() {
        return None
    }

    let descriptor = AtomDescriptor::from_cursor(cursor)?;
    if descriptor.pos >= mof {
        return None
    }

    cursor.move_to(descriptor.pos+descriptor.off);
    let neighboors = describe(cursor, mof);

    cursor.move_to(descriptor.pos+8);
    let children = describe(cursor, descriptor.off);

    let mut node = Node::from(descriptor);
    if let Some(children) = children {
        for child in children {
            node.add(child);
        }
    }

    let mut level_nodes = vec![node];
    if let Some(mut neighboors) = neighboors {
        level_nodes.append(&mut neighboors);
    }

    Some(level_nodes)
}

#[derive(Debug)]
pub struct AtomDescriptor
{
    pub off: usize,
    pub pos: usize,
    pub sig: String
}

impl AtomDescriptor
{
    fn from_cursor(cursor: &mut FileCursor) -> Option<AtomDescriptor> {
        let pos = cursor.offset;
        let off = read_offset(cursor);
        let sig = read_signature(cursor)?;
        Some(AtomDescriptor { sig, off, pos })
    }
}

fn read_offset(cursor: &mut FileCursor) -> usize {
    let off = cursor.read_exact::<4>();
    u32::from_be_bytes(off) as usize
}

// Allowed atom headers signatures
const VSGI: [&str; 171] = [ "ainf", "avcn", "bloc", "bpcc", "buff", "bxml", "ccid", "cdef", "clip", "cmap", "co64", "coin", "colr", "crgn", "crhd", "cslg", "ctab", "ctts", "cvru", "dinf", "dref", "dsgd", "dstg", "edts", "elst", "emsg", "evti", "fdel", "feci", "fecr", "fiin", "fire", "fpar", "free", "frma", "ftyp", "gitn", "grpi", "hdlr", "hmhd", "hpix", "icnu", "ID32", "idat", "ihdr", "iinf", "iloc", "imap", "imif", "infe", "infu", "iods", "iphd", "ipmc", "ipro", "iref", "jP$20$20", "jp2c", "jp2h", "jp2i", "kmat", "leva", "load", "loop", "lrcu", "m7hd", "matt", "mdat", "mdhd", "mdia", "mdri", "meco", "mehd", "mere", "meta", "mfhd", "mfra", "mfro", "minf", "mjhd", "moof", "moov", "mvcg", "mvci", "mvex", "mvhd", "mvra", "nmhd", "ochd", "odaf", "odda", "odhd", "odhe", "odrb", "odrm", "odtt", "ohdr", "padb", "paen", "pclr", "pdin", "pitm", "pnot", "prft", "pssh", "res$20", "resc", "resd", "rinf", "saio", "saiz", "sbgp", "schi", "schm", "sdep", "sdhd", "sdtp", "sdvp", "segr", "senc", "sgpd", "sidx", "sinf", "skip", "smhd", "srmb", "srmc", "srpp", "ssix", "stbl", "stco", "stdp", "sthd", "strd", "stri", "stsc", "stsd", "stsg", "stsh", "stss", "stsz", "stts", "styp", "stz2", "subs", "swtc", "tfad", "tfdt", "tfhd", "tfma", "tfra", "tibr", "tiri", "tkhd", "traf", "trak", "tref", "trex", "trgr", "trik", "trun", "udta", "uinf", "UITS", "ulst", "url", "uuid", "vmhd", "vwdi", "xml$20", "xml" ];

fn read_signature(cursor: &mut FileCursor) -> Option<String> {
    let sig = cursor.read_exact::<4>();
    match str::from_utf8(&sig) {
        Ok(value) => {
            let sig = String::from(value);
            if VSGI.iter().any(|i| &&sig == i) {
                Some(sig)
            } else {
                None
            }
        },
        Err(_) => None
    }
}
