use std::io;
use std::collections::BTreeSet;

fn main() {
    let n = read_values::<usize>()[0];
    let d = (0..n).map(|_| read_values::<usize>()[0]).collect::<BTreeSet<_>>();
    println!("{}", d.len());
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