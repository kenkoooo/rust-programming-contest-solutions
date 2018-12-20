use std::cmp;
use std::collections::BTreeSet;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };

    let n: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(1 << n);

    let mut top: Vec<(usize, usize)> = (0..(1 << n)).map(|i| (a[i], i)).collect();
    let mut ans = vec![0; 1 << n];
    for mask in 1..(1 << n) {
        let mut set = BTreeSet::new();
        set.insert(top[mask]);
        for pos in 0..n {
            let pos_mask = 1 << pos;
            let sub_mask = mask & (!pos_mask);
            set.insert(top[sub_mask]);
        }
        let mut iter = set.iter().rev();
        top[mask] = *iter.next().unwrap();

        let &(top2, _) = iter.next().unwrap();
        ans[mask] = cmp::max(ans[mask - 1], top[mask].0 + top2);
        println!("{}", ans[mask]);
    }
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
