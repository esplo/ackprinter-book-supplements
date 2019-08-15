#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

use num_bigint::BigUint;
use std::thread;

fn b(v: u32) -> BigUint {
    BigUint::from(v)
}

fn ackermann(m: u32, n: u32) -> BigUint {
    fn work(m: BigUint, n: BigUint, memo: &mut HashMap<(BigUint, BigUint), BigUint>) -> BigUint {
        if let Some(v) = memo.get(&(m.clone(), n.clone())) {
            (*v).clone()
        } else {
            if m == b(0) {
                let res = n.clone() + b(1);
                memo.insert((m, n), res.clone());
                res
            } else if n == b(0) {
                let res = work(m.clone() - b(1), b(1), memo);
                memo.insert((m, n), res.clone());
                res
            } else {
                let res = work(m.clone() - b(1), work(m.clone(), n.clone() - b(1), memo), memo);
                memo.insert((m, n), res.clone());
                res
            }
        }
    }


    fn run_in_thread(m: u32, n: u32) -> BigUint {
        let builder = thread::Builder::new()
            .name("large_stack_thread".into())
            .stack_size(3200 * 1024 * 1024);

        let handler = builder.spawn(move || {
            let mut memo = HashMap::new();
            work(b(m), b(n), &mut memo)
        }).unwrap();

        handler.join().unwrap()
    }

    run_in_thread(m, n)
}

fn biguint_ackermann(m: u32, n: u32) -> BigUint {
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


fn naive_ackermann(m: i32, n: i32) -> i32 {
    fn work(m: i32, n: i32) -> i32 {
        if m == 0 {
            n + 1
        } else if n == 0 {
            work(m - 1, 1)
        } else {
            work(m - 1, work(m, n - 1))
        }
    }

    fn run_in_thread(m: i32, n: i32) -> i32 {
        let builder = thread::Builder::new()
            .name("large_stack_thread".into())
            .stack_size(32 * 1024 * 1024);

        let handler = builder.spawn(move || {
            work(m, n)
        }).unwrap();

        handler.join().unwrap()
    }

    run_in_thread(m, n)
}


fn main() {
    println!("{}", ackermann(4, 2));
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
        assert_eq!(ackermann(4, 1), b(65533));
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
        b.iter(|| naive_ackermann(3, 7))
    }

    #[bench]
    fn biguint_bench(b: &mut Bencher) {
        b.iter(|| biguint_ackermann(3, 7))
    }

    #[bench]
    fn memo_bench(b: &mut Bencher) {
        b.iter(|| ackermann(3, 7))
    }
}
