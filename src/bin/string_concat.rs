#![cfg_attr(feature = "unstable", feature(test))]

fn main() {
}


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use self::test::Bencher;

    #[bench]
    fn plus(b: &mut Bencher) {
        let w = test::black_box("hoge");
        b.iter(|| {
            let mut s = String::new();
            s += w;
            s += "fuga";
            s
        });
    }

    #[bench]
    fn push_str(b: &mut Bencher) {
        let w = test::black_box("hoge");
        b.iter(|| {
            let mut s = String::new();
            s.push_str(w);
            s.push_str("fuga");
            s
        });
    }

    #[bench]
    fn cap_plus(b: &mut Bencher) {
        let w = test::black_box("hoge");
        b.iter(|| {
            let mut s = String::with_capacity(8);
            s.push_str(w);
            s.push_str("fuga");
            s
        });
    }

    #[bench]
    fn ary_join(b: &mut Bencher) {
        let w = test::black_box("hoge");
        b.iter(|| {
            [w, "fuga"].join("")
        });
    }
}
