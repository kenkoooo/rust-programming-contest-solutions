use std::io;

fn main() {
    let (n, l) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };

    let a = read_values::<usize>();
    let mut x = n;
    for i in 0..(n - 1) {
        if a[i] + a[i + 1] >= l {
            x = i;
            break;
        }
    }

    if x == n {
        println!("Impossible");
        return;
    }

    println!("Possible");
    for i in 0..x {
        println!("{}", i + 1);
    }
    for i in (x..(n - 1)).rev() {
        println!("{}", i + 1);
    }
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