extern crate spine;

use std::io::BufReader;

#[test]
fn animations_list() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::SpineDocument::new(BufReader::new(src)).ok().expect("cannot load example");

    assert!(doc.get_animations_list().get(0).unwrap() == &"walk" ||
        doc.get_animations_list().get(0).unwrap() == &"jump");
    assert!(doc.get_animations_list().get(1).unwrap() == &"walk" ||
        doc.get_animations_list().get(1).unwrap() == &"jump");

    assert!(doc.has_animation("walk"));
    assert!(doc.has_animation("jump"));
    assert!(!doc.has_animation("crawl"));
}

#[test]
fn skins_list() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::SpineDocument::new(BufReader::new(src)).unwrap();

    assert!(doc.get_skins_list().get(0).unwrap() == &"default");

    assert!(doc.has_skin("default"));
    assert!(!doc.has_skin("nonexisting"));
}

#[test]
fn possible_sprites() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::SpineDocument::new(BufReader::new(src)).unwrap();

    let mut results = doc.get_possible_sprites();
    results.sort();

    assert!(results == [
        "eyes", "eyes-closed", "head", "left-arm", "left-foot", "left-hand", "left-lower-leg",
        "left-shoulder", "left-upper-leg", "neck", "pelvis", "right-arm", "right-foot",
        "right-hand", "right-lower-leg", "right-shoulder", "right-upper-leg", "torso"
    ]);
}
