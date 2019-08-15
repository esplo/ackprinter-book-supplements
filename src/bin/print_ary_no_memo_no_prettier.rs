use std::env;

fn ary_prefixer(ary: &Vec<Vec<u32>>, v: u32) -> Vec<Vec<u32>> {
    ary.iter()
        .map(|e| [&[v], &e[..]].concat())
        .collect()
}

fn ackermann(m: u32, n: u32) {
    fn work(m: u32, n: u32) -> Vec<Vec<u32>> {
        let mut result = vec![vec![m, n]];

        if m == 0 {
            result.push(vec![n + 1]);
        } else if n == 0 {
            let ary = work(m - 1, 1);
            result.extend(ary);
        } else {
            let ary1 = work(m, n - 1);
            let pref_ary1 = ary_prefixer(&ary1, m - 1);
            // 最後の行 [m-1, v] は次の計算用
            result.extend_from_slice(&pref_ary1[0..pref_ary1.len() - 1]);

            let ary2 = work(m - 1, ary1[ary1.len() - 1][0]);
            result.extend(ary2);
        }

        result
    }

    let ary = work(m, n);
    for e in ary.iter() {
        println!("{}", e.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(","))
    }
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
        Ok((m, n)) => ackermann(m, n),
        Err(e) => { eprintln!("{}", e); }
    }
}
