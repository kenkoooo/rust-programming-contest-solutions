use std::io;
use std::cmp;

fn main() {
    let s = read_line().trim().bytes().map(|c| {
        c - "a".bytes().next().unwrap()
    }).collect::<Vec<_>>();
    let n = read_values::<usize>()[0];
    let lr = (0..n).map(|_| {
        let v = read_values::<usize>();
        let l = v[0] - 1;
        let r = v[1] - 1;

        if (n & 1) == 0 {
            // n%2==0
            if r < (n >> 1) {}
        } else {}
    }).collect::<Vec<_>>();
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}