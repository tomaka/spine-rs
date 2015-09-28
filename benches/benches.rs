#![feature(test)]

extern crate spine;
extern crate test;
extern crate clock_ticks;

use std::io::BufReader;

#[bench]
fn loading(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");

    bencher.iter(|| {
        spine::SpineDocument::new(BufReader::new(src))
    });
}

#[bench]
fn animation(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bytes!("../tests/example.json");
    let doc = spine::SpineDocument::new(BufReader::new(src)).unwrap();
    let animations = test::black_box(doc.get_animations());

    bencher.iter(|| {
        doc.calculate("default", animations.get("walk"), (clock_ticks::precise_time_ns() / 1000000) as f32 / 1000.0)
            // doc.calculate("default", Some("walk"), (clock_ticks::precise_time_ns() / 1000000) as f32 / 1000.0)
    })
}
