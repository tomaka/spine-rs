extern crate spine;

use std::io::BufReader;

#[test]
fn animations_names() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::skeleton::Skeleton::from_reader(BufReader::new(src)).unwrap();

    let names = doc.get_animations_names();

    assert!(names.get(0).unwrap() == &"walk" ||
        names.get(0).unwrap() == &"jump");
    assert!(names.get(1).unwrap() == &"walk" ||
        names.get(1).unwrap() == &"jump");

    assert!(names.contains(&"walk"));
    assert!(names.contains(&"jump"));
    assert!(!names.contains(&"crawl"));
}

#[test]
fn skins_names() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::skeleton::Skeleton::from_reader(BufReader::new(src)).unwrap();
    let skins = doc.get_skins_names();
    assert!(skins.get(0).unwrap() == &"default");

    assert!(skins.contains(&"default"));
    assert!(doc.get_skin("default").is_ok());

    assert!(!skins.contains(&"nonexisting"));
    assert!(doc.get_skin("nonexisting").is_err());
}

#[test]
fn attachement_names() {
    let src: &[u8] = include_bytes!("example.json");
    let doc = spine::skeleton::Skeleton::from_reader(BufReader::new(src)).unwrap();

    let mut results = doc.get_attachments_names();
    results.sort();

    assert!(results == [
        "eyes", "eyes-closed", "head", "left-arm", "left-foot", "left-hand", "left-lower-leg",
        "left-shoulder", "left-upper-leg", "neck", "pelvis", "right-arm", "right-foot",
        "right-hand", "right-lower-leg", "right-shoulder", "right-upper-leg", "torso"
    ]);
}
