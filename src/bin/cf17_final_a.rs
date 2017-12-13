use std::io;

fn main() {
    let labels = (0..(1 << 4)).map(|mask| {
        let mut label = "".to_owned();
        if ((1 << 0) & mask) != 0 {
            label.push('A');
        }
        label.push('K');
        label.push('I');
        label.push('H');
        if ((1 << 1) & mask) != 0 {
            label.push('A');
        }
        label.push('B');
        if ((1 << 2) & mask) != 0 {
            label.push('A');
        }
        label.push('R');
        if ((1 << 3) & mask) != 0 {
            label.push('A');
        }
        label
    }).collect::<Vec<_>>();

    let s = read_line().trim().to_owned();

    if labels.contains(&s) {
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