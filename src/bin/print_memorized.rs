use std::collections::HashMap;
use std::env;

fn s_converter(s: &str, v: u32) -> String {
    let tokens: Vec<&str> = s.split('=').collect();
    // Capacityを指定
    let mut result = Vec::with_capacity(tokens.len() - 1);

    for t in &tokens[..tokens.len() - 1] {
        // 先頭には 'A(v,' をつけ、後ろに ')' をつける
        result.push(format!("A({},{})", v, t))
    }

    result.join("=")
}


fn ackermann(m: u32, n: u32) -> String {
    fn work(m: u32, n: u32, memo: &mut HashMap<(u32, u32), (u32, String)>) -> (u32, String) {
        if let Some(memov) = memo.get(&(m, n)) {
            (*memov).clone()
        } else {
            if m == 0 {
                let (r1, r2) = (n + 1, format!("A({},{})={}", m, n, n + 1));
                memo.insert((m, n), (r1, r2.clone()));
                (r1, r2)
            } else if n == 0 {
                let (v, s) = work(m - 1, 1, memo);
                let (r1, r2) = (v, format!("A({},{})={}", m, n, s));
                memo.insert((m, n), (r1, r2.clone()));
                (r1, r2)
            } else {
                let (v, s) = work(m, n - 1, memo);
                let (v2, s2) = work(m - 1, v, memo);
                let (r1, r2) = (v2, format!("A({},{})={}={}", m, n, s_converter(&s, m - 1), s2));
                memo.insert((m, n), (r1, r2.clone()));
                (r1, r2)
            }
        }
    }

    let mut memo = HashMap::new();
    let (_, s) = work(m, n, &mut memo);
    s
}


fn inputs(args: &Vec<String>) -> Result<(u32, u32), String> {
    fn read_int(s: &str) -> Result<u32, String> {
        s.parse::<u32>().map_err(|err| err.to_string())
    }

    if args.len() != 3 {
        Err(format!("invalid number of arguments: {}", args.len()))
    } else {
        let u: Vec<u32> = args.iter().skip(1).flat_map(|s| read_int(s)).collect();
        if u.len() != 2 {
            Err(format!("invalid digit found in string"))
        } else {
            Ok((u[0], u[1]))
        }
    }
}

fn main() {
    match inputs(&env::args().map(|s| s).collect()) {
        Ok((m, n)) => println!("{}", ackermann(m, n)),
        Err(e) => eprintln!("{}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_owned(s: Vec<&str>) -> Vec<String> {
        s.iter().map(|&s| s.to_owned()).collect()
    }

    #[test]
    fn valid_input() {
        assert_eq!(inputs(&to_owned(vec!["./ackermann", "2", "4"])), Ok((2, 4)));
    }

    #[test]
    fn invalid_input() {
        assert_eq!(inputs(&to_owned(vec!["./ackermann", "2", "hoge"])), Err("invalid digit found in string".to_owned()));
        assert_eq!(inputs(&to_owned(vec!["./ackermann", "2", "4", "9"])), Err("invalid number of arguments: 4".to_owned()));
    }

    #[test]
    fn path1() {
        assert_eq!(ackermann(0, 0), "A(0,0)=1");
    }

    #[test]
    fn path2() {
        assert_eq!(ackermann(1, 0), "A(1,0)=A(0,1)=2");
    }

    #[test]
    fn path3() {
        assert_eq!(ackermann(1, 1), "A(1,1)=A(0,A(1,0))=A(0,A(0,1))=A(0,2)=3");
        assert_eq!(ackermann(1, 2), "A(1,2)=A(0,A(1,1))=A(0,A(0,A(1,0)))=A(0,A(0,A(0,1)))=A(0,A(0,2))=A(0,3)=4");
        assert_eq!(ackermann(2, 1), "A(2,1)=A(1,A(2,0))=A(1,A(1,1))=A(1,A(0,A(1,0)))=A(1,A(0,A(0,1)))=A(1,A(0,2))=A(1,3)=A(0,A(1,2))=A(0,A(0,A(1,1)))=A(0,A(0,A(0,A(1,0))))=A(0,A(0,A(0,A(0,1))))=A(0,A(0,A(0,2)))=A(0,A(0,3))=A(0,4)=5");
    }

    #[test]
    fn test_s_converter() {
        let v = "A(1,1)=A(0,A(1,0))=A(0,A(0,1))=A(0,2)=3";
        assert_eq!(s_converter(v, 0), "A(0,A(1,1))=A(0,A(0,A(1,0)))=A(0,A(0,A(0,1)))=A(0,A(0,2))");
    }

    #[test]
    fn test_ary_prefixer() {
        let v = vec![
            vec![1, 1],
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![0, 2],
            vec![3]
        ];
        let res = vec![
            vec![0, 1, 1],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
            vec![0, 0, 2],
        ];
        assert_eq!(ary_prefixer(&v, 0), res);
    }
}


