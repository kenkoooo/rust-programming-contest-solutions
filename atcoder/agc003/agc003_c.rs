use std::io;
use std::collections::BTreeMap;

fn main() {
    let n = read_values::<usize>()[0];
    let mut array = (0..n).map(|_| read_values::<usize>()[0]).collect::<Vec<_>>();
    let mut map = BTreeMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            map.insert(array[i], true);
        } else {
            map.insert(array[i], false);
        }
    }

    array.sort();
    let mut count = 0;
    for i in 0..n {
        if i % 2 != 0 && *map.get(&array[i]).unwrap() {
            count += 1;
        }
    }
    println!("{}", count);
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