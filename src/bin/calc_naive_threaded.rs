use std::thread;

fn ackermann(m: i32, n: i32) -> i32 {
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
    println!("{}", ackermann(1, 0));
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn a0() {
        for i in 0..10 {
            assert_eq!(ackermann(0, i), i + 1);
        }
    }

    #[test]
    fn a1() {
        for i in 0..10 {
            assert_eq!(ackermann(1, i), i + 2);
        }
    }

    #[test]
    fn a2() {
        for i in 0..10 {
            assert_eq!(ackermann(2, i), 2 * i + 3);
        }
    }

    #[test]
    fn a3() {
        for i in 0..10 {
            let ui: u32 = i.try_into().unwrap();
            assert_eq!(ackermann(3, i), 2_u32.pow(ui + 3) - 3);
        }
    }

    #[test]
    fn a4() {
        assert_eq!(ackermann(4, 0), 13);
        assert_eq!(ackermann(4, 1), 65533);
//        assert_eq!(ackermann(4, 2), 0);
    }
}
