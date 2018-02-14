fn main() {
    let (x, k) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let mut y = 1;
    for _ in 0..k {
        y *= 10;
    }
    if x / y > 0 {
        println!("{}", x / y * y + y);
    } else {
        println!("{}", y);
    }
}

fn read_line() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug, {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}