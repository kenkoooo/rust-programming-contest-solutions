use std::io;

fn main() {
    let n = read_values::<usize>()[0];
    let mut cur = (0, 0);
    let mut time = 0;
    for _ in 0..n {
        let (t, x, y) = {
            let v = read_values::<i64>();
            (v[0], v[1], v[2])
        };

        let duration = t - time;
        let (cx, cy) = cur;
        let length = (cx - x).abs() + (cy - y).abs();
        if length > duration {
            println!("No");
            return;
        }
        if (duration - length) % 2 != 0 {
            println!("No");
            return;
        }
        time = t;
        cur = (x, y)
    }
    println!("Yes");
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