extern crate spine;
extern crate test;
extern crate time;

use std::io::BufReader;

#[bench]
fn loading(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bin!("../tests/example.json");

    bencher.iter(|| {
        spine::SpineDocument::new(&mut BufReader::new(src))
    });
}

#[bench]
fn animation(bencher: &mut test::Bencher) {
    let src: &[u8] = include_bin!("../tests/example.json");
    let doc = spine::SpineDocument::new(&mut BufReader::new(src)).unwrap();

    bencher.iter(|| {
        doc.calculate("default", Some("walk"), (time::precise_time_ns() / 1000000) as f32 / 1000.0)
    })
}
