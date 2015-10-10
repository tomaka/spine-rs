#![feature(test)]

extern crate spine;
extern crate test;
extern crate clock_ticks;

use std::io::BufReader;

#[bench]
fn loading_json(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");

    bencher.iter(|| {
        spine::SpineDocument::new(BufReader::new(src))
    });
}

#[bench]
fn loading_skeleton(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");
    bencher.iter(|| {
        spine::Skeleton::from_reader(BufReader::new(src))
    });
}

#[bench]
fn animation(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::SpineDocument::new(BufReader::new(src)).unwrap();

    bencher.iter(|| {
        doc.calculate("default", Some("walk"), (clock_ticks::precise_time_ns() / 1000000) as f32 / 1000.0)
    })
}

#[bench]
fn animation_skeleton(bencher: &mut test::Bencher) {

    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::Skeleton::from_reader(BufReader::new(src)).unwrap();

    bencher.iter(|| {
        if let Ok(mut anim) = doc.iter("default", Some("walk"), 0.01) {
            anim.next();
        }
    })
}


#[bench]
fn animation_all(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::SpineDocument::new(BufReader::new(src)).unwrap();

    bencher.iter(|| {
        (0..100).map(|t| doc.calculate("default", Some("walk"), t as f32 /100f32)).collect::<Vec<_>>();
    })
}

#[bench]
fn animation_skeleton_all(bencher: &mut test::Bencher) {

    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::Skeleton::from_reader(BufReader::new(src)).unwrap();

    bencher.iter(|| {
        if let Ok(mut anim) = doc.iter("default", Some("walk"), 0.01) {
            anim.nth(100);
        }
    })
}
