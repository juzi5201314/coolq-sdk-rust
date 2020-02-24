#![feature(test)]

use test::Bencher;

extern crate test;

#[bench]
fn bench_escape_cqcode(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10000 {
            escape(String::from(test::black_box("&#91;CQ:at&#44;qq=1230&#93;")));
        }
    })
}

#[test]
fn test() {
    dbg!(escape("&#91;CQ:at&#44;qq=1230&#93;".to_string()));
}

fn escape(s: String) -> String {
    s.replace("&amp;", "&")
        .replace("&#91;", "[")
        .replace("&#93;", "]")
        .replace("&#44;", ",")
}
