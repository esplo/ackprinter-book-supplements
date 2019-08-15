use std::collections::HashMap;
use std::env;

fn ary_prefixer(ary: &Vec<Vec<u32>>, v: u32) -> Vec<Vec<u32>> {
    ary.iter()
        .map(|e| [&[v], &e[..]].concat())
        .collect()
}

fn prettier(v: &Vec<Vec<u32>>) -> String {
    let mut s = String::new();
    let (last, elem) = v.split_last().unwrap();

    for r in elem.into_iter() {
        let (rlast, relem) = r.split_last().unwrap();
        let suf = relem.len();

        for c in relem.into_iter() {
            s.push_str("A(");
            s.push_str(&format!("{}", c));
            s.push_str(",");
        }
        s.push_str(&format!("{}", rlast));
        s.push_str(&")".repeat(suf));
        s.push_str("=");
    }
    // protective
    if !last.is_empty() {
        s.push_str(&format!("{}", last[0]));
    }
    s
}

fn ackermann(m: u32, n: u32) -> String {
    fn work(m: u32, n: u32, memo: &mut HashMap<(u32, u32), Vec<Vec<u32>>>) -> Vec<Vec<u32>> {
        if let Some(memov) = memo.get(&(m, n)) {
            (*memov).clone()
        } else {
            let mut result = vec![vec![m, n]];

            if m == 0 {
                result.push(vec![n + 1]);
            } else if n == 0 {
                let ary = work(m - 1, 1, memo);
                result.extend(ary);
            } else {
                let ary1 = work(m, n - 1, memo);
                let pref_ary1 = ary_prefixer(&ary1, m - 1);
                // 最後の行 [m-1, v] は次の計算用
                result.extend_from_slice(&pref_ary1[0..pref_ary1.len() - 1]);

                let ary2 = work(m - 1, ary1[ary1.len() - 1][0], memo);
                result.extend(ary2);
            }

            memo.insert((m, n), result.clone());
            result
        }
    }

    let mut memo = HashMap::new();
    let ary = work(m, n, &mut memo);
    prettier(&ary)
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
}


