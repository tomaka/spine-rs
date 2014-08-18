extern crate spine;

use std::io::BufReader;

#[test]
fn simple() {
    let src: &[u8] = include_bin!("example.json");

    let doc = spine::SpineDocument::new(&mut BufReader::new(src)).unwrap();
}
