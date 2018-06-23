use std::io;
use std::cmp;

fn main() {
    let (n, k) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };

    let mut tree = vec![Vec::new(); n];
    let mut edges = Vec::new();
    for _ in 0..(n - 1) {
        let v = read_values::<usize>();
        let from = v[0] - 1;
        let to = v[1] - 1;
        tree[from].push(to);
        tree[to].push(from);

        edges.push((from, to));
    }

    if k % 2 == 1 {
        let mut max = 0;
        for edge in &edges {
            let (from, to) = *edge;
            let ans = dfs(&tree, from, to, 0, k / 2) + dfs(&tree, to, from, 0, k / 2);
            max = cmp::max(max, ans);
        }
        println!("{}", n - max);
    } else {
        let mut max = 0;
        for v in 0..n {
            let ans = dfs(&tree, v, n, 0, k / 2);
            max = cmp::max(ans, max);
        }
        println!("{}", n - max);
    }
}

fn dfs(tree: &Vec<Vec<usize>>, v: usize, p: usize, dist: usize, k: usize) -> usize {
    if dist > k {
        return 0;
    }
    let mut count = 1;
    for next in tree[v].iter() {
        if *next == p {
            continue;
        }
        count += dfs(tree, *next, v, dist + 1, k);
    }
    count
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