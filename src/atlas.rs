//! Module to import .atlas files

use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::fmt;
use std::error::Error;
use std::str::ParseBoolError;

/// atlas texture
pub struct Texture {
    /// name
    pub name: String,
    /// rotate
    pub rotate: bool,
    /// position
    pub xy: (u16, u16),
    /// size
    pub size: (u16, u16),
    /// orig
    pub orig: (u16, u16),
    /// offset
    pub offset: (u16, u16),
    /// index
    pub index: i16,
}

/// Iterator to parse attachments from a common image
pub struct Atlas<R: Read> {
    /// file
    pub file: String,
    /// format
    pub format: String,
    /// filter
    pub filter: String,
    /// repeat
    pub repeat: String,
    lines: Lines<BufReader<R>>
}

impl<R: Read> Atlas<R> {

    /// consumes a reader on .atlas file and create a Atlas iterator
    pub fn from_reader(reader: R) -> Result<Atlas<R>, AtlasError> {
        let mut lines = BufReader::new(reader).lines();
        while let Some(line) = lines.next() {
            let line = try!(line);
            if line.trim().len() > 0 {

                let file = line;
                let val = try!(next_line(&mut lines));
                let format = val["format:".len()..].trim().to_owned();
                let val = try!(next_line(&mut lines));
                let filter = val["filter:".len()..].trim().to_owned();
                let val = try!(next_line(&mut lines));
                let repeat = val["repeat:".len()..].trim().to_owned();

                return Ok(Atlas {
                    file: file,
                    format: format,
                    filter: filter,
                    repeat: repeat,
                    lines: lines
                });
            }
        }
        Err(AtlasError::Unexpected("cannot parse headers"))
    }

    fn read_texture(&mut self, name: &str) -> Result<Texture, AtlasError> {
        let rotate = {
            let line = try!(next_line(&mut self.lines));
            try!(line.trim_left()["rotate:".len()..].trim().parse())
        };
        let mut tuples = Vec::with_capacity(4);
        for pattern in ["xy:", "size:", "orig:", "offset:"].into_iter() {
            let val = try!(self.parse_tuple(pattern.len()));
            tuples.push(val);
        }
        let index = {
            let line = try!(next_line(&mut self.lines));
            try!(line.trim_left()["index:".len()..].trim().parse())
        };
        Ok(Texture {
            name: name.to_owned(),
            rotate: rotate,
            xy: tuples[0],
            size: tuples[1],
            orig: tuples[2],
            offset: tuples[3],
            index: index,
        })
    }

    fn parse_tuple(&mut self, offset: usize) -> Result<(u16, u16), AtlasError> {
        let line = try!(next_line(&mut self.lines));
        let mut tuple = Vec::with_capacity(2);
        for s in line.trim_left()[offset..].split(',').take(2) {
            let a = try!(s.trim().parse());
            tuple.push(a);
        }
        if tuple.len() != 2 {
            Err(AtlasError::Unexpected("tuple"))
        } else {
            Ok((tuple[0], tuple[1]))
        }
    }
}

fn next_line<R: Read>(lines: &mut Lines<BufReader<R>>) -> Result<String, AtlasError> {
    match lines.next() {
        Some(Ok(line)) => Ok(line),
        Some(Err(e)) => Err(AtlasError::from(e)),
        None => Err(AtlasError::Unexpected("EOF"))
    }
}

impl<R: Read> Iterator for Atlas<R> {
    type Item = Result<Texture, AtlasError>;
    fn next(&mut self) -> Option<Result<Texture, AtlasError>> {
        loop {
            return match self.lines.next() {
                Some(Ok(name)) => {
                    let name = name.trim();
                    if name.len() == 0 { continue; }
                    Some(self.read_texture(name.trim()))
                },
                Some(Err(e)) => Some(Err(AtlasError::from(e))),
                None         => None
            }
        }
    }
}

/// Atlas errors
pub enum AtlasError {
    /// io error
    IoError(::std::io::Error),
    /// unexpected error, with descriptiom
    Unexpected(&'static str),
    /// error when parsing u16 or i16
    ParseIntError(::std::num::ParseIntError),
    /// error when parsing boolean
    ParseBoolError(::std::str::ParseBoolError)
}

impl fmt::Display for AtlasError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, formatter)
    }
}

impl Error for AtlasError {
    fn description(&self) -> &str {
        match *self {
            AtlasError::ParseIntError(_) => "error parsing integer",
            AtlasError::ParseBoolError(_) => "error parsing boolean",
            AtlasError::Unexpected(_) => "unexpected error",
            AtlasError::IoError(_) => "error reading atlas file",
        }
    }
}

impl fmt::Debug for AtlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AtlasError::ParseIntError(ref e) => write!(f, "Cannot parse integer: {:?}", e),
            AtlasError::ParseBoolError(ref e) => write!(f, "Cannot parse boolean: {:?}", e),
            AtlasError::Unexpected(s) => write!(f, "Unexpected error: {}", s),
            AtlasError::IoError(ref e) => write!(f, "Error reading atlas file: {:?}", e),
        }
    }
}

impl From<::std::io::Error> for AtlasError {
    fn from(error: ::std::io::Error) -> AtlasError {
        AtlasError::IoError(error)
    }
}

impl From<::std::num::ParseIntError> for AtlasError {
    fn from(error: ::std::num::ParseIntError) -> AtlasError {
        AtlasError::ParseIntError(error)
    }
}

impl From<ParseBoolError> for AtlasError {
    fn from(error: ParseBoolError) -> AtlasError {
        AtlasError::ParseBoolError(error)
    }
}
