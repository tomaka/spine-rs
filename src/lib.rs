extern crate cgmath;
extern crate serialize;

use cgmath::{Matrix4};
use serialize::json;

mod format;

pub struct SpineDocument {
    source: format::Document,
}

impl SpineDocument {
    pub fn new<R: Reader>(reader: &mut R) -> Result<SpineDocument, String> {

        let document = try!(json::from_reader(reader).map_err(|e| e.to_string()));
        let mut decoder = json::Decoder::new(document);
        let document = try!(serialize::Decodable::decode(&mut decoder).map_err(|e| e.to_string()));

        Ok(SpineDocument {
            source: document
        })
    }

    /// Returns the list of all animations in this document.
    pub fn get_animations_list(&self) -> Vec<&str> {
        unimplemented!()
    }
}
