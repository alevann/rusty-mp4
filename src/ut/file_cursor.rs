use std::fs;
use std::iter::FromIterator;

pub struct FileCursor
{
    pub offset: usize,
    pub count: usize,
    data: Vec<u8>,
}

impl FileCursor {
    pub fn from(path: String) -> FileCursor {
        let data = fs::read(path).expect("Unable to read file");
        FileCursor {
            offset: 0,
            count: data.len(),
            data
        }
    }

    pub fn consumed(&self) -> bool {
        self.offset >= self.count
    }

    pub fn read_exact<const S: usize>(&mut self) -> [u8; S] {
        let mut out = [0 as u8; S];
        for (i, v) in self.data[self.offset..self.offset+S]
                            .iter().cloned().enumerate() {
            out[i] = v;
        }
        self.offset += S;
        out
    }

    pub fn read(&mut self, count: usize) -> Vec<u8> {
        let out = Vec::from_iter(
            self.data[self.offset..self.offset+count].iter().cloned());
        self.offset += count;
        out
    }

    pub fn move_to(&mut self, to: usize) {
        self.offset = to
    }
}