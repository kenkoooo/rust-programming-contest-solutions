use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a: Vec<Vec<_>> = (0..n)
        .map(|_| (1..n).map(|_| sc.read::<usize>() - 1).collect())
        .collect();
    let mut pos = vec![0; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        let j = a[i][0];
        if a[j][0] == i && i < j {
            q.push_back((i, j, 1));
        }
    }

    let mut t = 0;

    while let Some((i, j, time)) = q.pop_front() {
        t = max(t, time);
        pos[i] += 1;
        if pos[i] < n - 1 {
            let j = a[i][pos[i]];
            if a[j][pos[j]] == i {
                q.push_back((i, j, time + 1));
            }
        }

        let i = j;
        pos[i] += 1;
        if pos[i] < n - 1 {
            let j = a[i][pos[i]];
            if a[j][pos[j]] == i {
                q.push_back((i, j, time + 1));
            }
        }
    }

    if pos.into_iter().all(|x| x == n - 1) {
        println!("{}", t);
    } else {
        println!("-1");
    }
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
