use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;

pub struct Texture {
    pub rotate: bool,
    pub xy: (u16, u16),
    pub size: (u16, u16),
    pub orig: (u16, u16),
    pub offset: (u16, u16),
    pub index: i16,
}

/// Iterator to parse attachments from a common image
pub struct Atlas {
    lines: Lines<BufReader<File>>
}

impl Atlas {

    /// Reads a .atlas file and create a Atlas iterator
    pub fn from_file(src: &str) -> Result<Atlas, ::std::io::Error> {
        let f = try!(File::open(src));
        let reader = BufReader::new(f);
        let lines = reader.lines();
        Ok(Atlas {
            lines: lines
        })
    }

    fn next_texture(&mut self) -> Option<(String, String)> {
        let mut old_line = String::new();
        loop {
            if let Some(Ok(line)) = self.lines.next() {
                if line.starts_with("\t") || line.starts_with(" ") {
                    return Some((old_line, line));
                }
                old_line = line;
            } else {
                return None;
            }
        }
    }

    fn read_texture(&mut self, line: &str) -> Texture  {
        let rotate = line.trim()["rotate:".len()..].trim().parse().unwrap();
        let mut line = self.lines.next().unwrap().unwrap();
        let xy = parse_tuple(&line.trim()["xy:".len()..]);
        line = self.lines.next().unwrap().unwrap();
        let size = parse_tuple(&line.trim()["size:".len()..]);
        line = self.lines.next().unwrap().unwrap();
        let orig = parse_tuple(&line.trim()["orig:".len()..]);
        line = self.lines.next().unwrap().unwrap();
        let offset = parse_tuple(&line.trim()["offset:".len()..]);
        line = self.lines.next().unwrap().unwrap();
        let index = line.trim()["index:".len()..].trim().parse().unwrap();
        Texture {
            rotate: rotate,
            xy: xy,
            size: size,
            orig: orig,
            offset: offset,
            index: index,
        }
    }
}

impl Iterator for Atlas {
    type Item = (String, Texture);
    fn next(&mut self) -> Option<(String, Texture)> {
        if let Some((name, line)) = self.next_texture() {
            let txt = self.read_texture(&line);
            Some((name, txt))
        } else {
            None
        }
    }
}

fn parse_tuple(s: &str) -> (u16, u16) {
    let mut splits = s.split(',');
    (splits.next().unwrap().trim().parse().unwrap(),
     splits.next().unwrap().trim().parse().unwrap())
}
