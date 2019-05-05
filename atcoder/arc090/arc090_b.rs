use std::cmp;
use std::collections::VecDeque;

fn main() {
    let (n, m) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };

    let mut map: Vec<Vec<(usize, i64)>> = (0..n).map(|_| Vec::new()).collect::<Vec<_>>();
    for _ in 0..m {
        let v: Vec<usize> = read_values::<usize>();
        let l = v[0] - 1;
        let r: usize = v[1] - 1;
        let d = v[2] as i64;
        map[l].push((r, d));
        map[r].push((l, -d));
    }


    let mut x = vec![0; n];
    let mut checked = vec![false; n];
    let mut queue = VecDeque::new();

    for v in 0..n {
        if !checked[v] {
            queue.push_back(v);
            checked[v] = true;
        }

        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            for t in &map[i] {
                let (to, d) = *t;
                if checked[to] {
                    if x[to] != x[i] + d {
                        println!("No");
                        return;
                    }
                } else {
                    checked[to] = true;
                    x[to] = x[i] + d;
                    queue.push_back(to);
                }
            }
        }
    }


    let mut min = 0;
    let mut max = 0;
    for d in &x {
        min = cmp::min(min, *d);
        max = cmp::max(max, *d);
    }
    if max - min > 1000000000 {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn read_line() -> String {
    let stdin = std::io::stdin();
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
