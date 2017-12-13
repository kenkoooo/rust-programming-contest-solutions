use std::io;
use std::cmp;

fn main() {
    let n = read_values::<usize>()[0];
    let d = read_values::<i32>();

    let mut count = vec![0; 13];
    for e in &d {
        count[(*e as usize)] += 1;
    }

    if count[0] >= 1 {
        println!("0");
        return;
    }

    for c in &count {
        if *c >= 3 {
            println!("0");
            return;
        }
    }
    if count[12] >= 2 {
        println!("0");
        return;
    }

    let mut fixed = Vec::<i32>::new();
    let mut d = Vec::<i32>::new();
    for i in 1..13 {
        if count[i] == 2 {
            fixed.push(i as i32);
        } else if count[i] == 1 {
            d.push(i as i32);
        }
    }

    let mut max = 0;
    for mask in 0..(1 << d.len()) {
        let mut v = Vec::new();
        for i in 0..d.len() {
            if ((1 << i) & mask) != 0 {
                v.push(d[i]);
            } else {
                v.push(-d[i]);
            }
        }

        for f in &fixed {
            v.push(*f);
            v.push(-(*f));
        }
        v.push(0);

        v.sort();
        let mut min = 12;
        for i in 0..v.len() {
            if i == v.len() - 1 {
                min = cmp::min(min, v[0] + 24 - v[i]);
            } else {
                min = cmp::min(min, v[i + 1] - v[i]);
            }
        }

        max = cmp::max(max, min);
    }
    println!("{}", max);
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