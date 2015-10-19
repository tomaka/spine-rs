extern crate spine;

use std::io::BufReader;

#[test]
fn test() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::Skeleton::from_reader(BufReader::new(src)).unwrap();

    let anim = doc.get_animated_skin("default", None).unwrap();
    let _ = anim.interpolate(0.0);
}
