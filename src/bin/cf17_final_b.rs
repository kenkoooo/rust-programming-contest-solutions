use std::io;
use std::cmp;

fn main() {
    let s = read_line().trim().to_owned();
    let (mut a, mut b, mut c) = (0, 0, 0);

    for ch in s.chars(){
        if ch == 'a' {
            a += 1;
        } else if ch == 'b' {
            b += 1;
        } else {
            c += 1;
        }
    }
    let max = cmp::max(a, cmp::max(b, c));
    let min = cmp::min(a, cmp::min(b, c));
    if max - min <= 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read<T>() -> T
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    read_line().trim().parse().unwrap()
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