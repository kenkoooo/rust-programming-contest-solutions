use std::io;
use std::collections::BTreeSet;

fn main() {
    let (n, mut h) = {
        let v = read_values::<usize>();
        (v[0], v[1] as i64)
    };
    let mut vec = Vec::new();
    for _ in 0..n {
        let v = read_values::<i64>();
        vec.push((v[0], true));
        vec.push((v[1], false));
    }
    vec.sort();

    let mut ans = 0;
    for i in (0..vec.len()).rev() {
        let (atk, inf) = vec[i];
        if inf {
            ans += (h + atk - 1) / atk;
            break;
        } else {
            ans += 1;
            h -= atk;
        }
        if h <= 0 {
            break;
        }
    }
    println!("{}", ans);
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}