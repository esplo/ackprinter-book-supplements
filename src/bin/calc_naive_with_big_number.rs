#![cfg_attr(feature = "unstable", feature(test))]

use num_bigint::BigUint;

fn b(v: u32) -> BigUint {
    BigUint::from(v)
}

fn ackermann(m: u32, n: u32) -> BigUint {
    fn work(m: BigUint, n: BigUint) -> BigUint {
        if m == b(0) {
            n + b(1)
        } else if n == b(0) {
            work(m - b(1), b(1))
        } else {
            work(m.clone() - b(1), work(m, n - b(1)))
        }
    }

    work(b(m), b(n))
}

fn naive_ackermann(m: u32, n: u32) -> u32 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        naive_ackermann(m - 1, 1)
    } else {
        naive_ackermann(m - 1, naive_ackermann(m, n - 1))
    }
}

fn main() {
    println!("{}", naive_ackermann(3, 10));
    println!("{}", ackermann(3, 10));
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn a0() {
        for i in 0..10 {
            assert_eq!(ackermann(0, i), b(i + 1));
        }
    }

    #[test]
    fn a1() {
        for i in 0..10 {
            assert_eq!(ackermann(1, i), b(i + 2));
        }
    }

    #[test]
    fn a2() {
        for i in 0..10 {
            assert_eq!(ackermann(2, i), b(2 * i + 3));
        }
    }

    #[test]
    fn a3() {
        for i in 0..10 {
            let ui: u32 = i.try_into().unwrap();
            assert_eq!(ackermann(3, i), b(2_u32.pow(ui + 3) - 3));
        }
    }

    #[test]
    fn a4() {
        assert_eq!(ackermann(4, 0), b(13));
//        assert_eq!(ackermann(4, 1), b(65535));
//        assert_eq!(ackermann(4, 2), b(0));
    }

    #[test]
    fn conv_from_u32_biguint() {
        assert_eq!(b(5), BigUint::new(vec![5]));
    }
}


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;

    use self::test::Bencher;

    #[bench]
    fn naive_bench(b: &mut Bencher) {
        b.iter(|| naive_ackermann(4, 1))
    }

    #[bench]
    fn hoge(b: &mut Bencher) {
        b.iter(|| {
            (0..1000).fold(0, |old, new| old ^ new)
        })
    }

    #[bench]
    fn biguint_bench(b: &mut Bencher) {
        b.iter(|| ackermann(3, 2))
    }
}
