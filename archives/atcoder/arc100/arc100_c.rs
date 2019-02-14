use std::cmp;
use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(1 << n);

    let mut top = a
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .collect::<Vec<(usize, usize)>>();

    let mut ans = vec![0; 1 << n];
    for mask in 1..(1 << n) {
        let mut set = BTreeSet::new();
        set.insert(top[mask]);
        for pos in 0..n {
            let pos_mask = 1 << pos;
            set.insert(top[mask & (!pos_mask)]);
        }

        let mut iter = set.into_iter().rev();
        top[mask] = iter.next().unwrap();
        let second = iter.next().unwrap();
        ans[mask] = cmp::max(ans[mask - 1], top[mask].0 + second.0);
        println!("{}", ans[mask]);
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
