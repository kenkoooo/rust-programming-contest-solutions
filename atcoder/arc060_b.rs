use std::io;
use std::collections::BTreeMap;

fn f(b: usize, n: usize) -> usize {
    if n < b {
        n
    } else {
        f(b, n / b) + n % b
    }
}

fn main() {
    let n = read_values::<usize>()[0];
    let s = read_values::<usize>()[0];

    if n == s {
        println!("{}", n + 1);
        return;
    }

    for b in 2..400000 {
        if f(b, n) == s {
            println!("{}", b);
            return;
        }
    }

    // x  + y = s
    // bx + y = n
    // => (b-1)x = n - s
    if n <= s {
        println!("-1");
        return;
    }
    let ns = n - s;
    let mut divisors = Vec::new();
    for d in 2..400000 {
        if d * d > ns {
            break;
        }
        if ns % d == 0 {
            divisors.push(d);
            divisors.push(ns / d);
        }
    }
    divisors.push(ns);
    divisors.sort();

    for a in &divisors {
        let b = *a + 1;
        if f(b, n) == s {
            println!("{}", b);
            return;
        }
    }
    println!("-1");
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