#![feature(test)]

extern crate spine;
extern crate test;
extern crate clock_ticks;

use std::io::BufReader;

#[bench]
fn loading_json(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");
    bencher.iter(|| {
        spine::skeleton::Skeleton::from_reader(BufReader::new(src))
    });
}

#[bench]
fn animation(bencher: &mut test::Bencher) {

    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::skeleton::Skeleton::from_reader(BufReader::new(src)).unwrap();

    bencher.iter(|| {
        if let Ok(anim) = doc.get_animated_skin("default", Some("walk")) {
            anim.interpolate(0.01);
        }
    })
}

#[bench]
fn animation_all(bencher: &mut test::Bencher) {

    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::skeleton::Skeleton::from_reader(BufReader::new(src)).unwrap();

    bencher.iter(|| {
        if let Ok(anim) = doc.get_animated_skin("default", Some("walk")) {
            for sprites in anim.run(0.01).take(100) {
                for a in sprites {
                    let _ = a;
                }
            }
        }
    })
}
