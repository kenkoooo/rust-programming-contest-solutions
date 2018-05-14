use std::io;

fn main() {
    let (n, y) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    for i in 0..2001 {
        if i > n || 10000 * i > y {
            continue;
        }
        for j in 0..4001 {
            if n < i + j {
                continue;
            }
            let sum = 10000 * i + 5000 * j + (n - i - j) * 1000;
            if sum == y {
                println!("{} {} {}", i, j, n - i - j);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}
