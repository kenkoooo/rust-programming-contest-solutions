use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(1 << n);
    let mut max2: Vec<Vec<(u64, usize)>> =
        a.iter().enumerate().map(|(i, &a)| vec![(a, i)]).collect();
    for mask in 0..(1 << n) {
        for i in 0..n {
            let next = mask | (1 << i);
            let from = max2[mask].clone();
            max2[next].extend(from);
            max2[next].sort();
            max2[next].reverse();
            max2[next].dedup();
            max2[next].resize(2, (0, 0));
        }
    }

    let mut ans = vec![0; 1 << n];
    for mask in 1..(1 << n) {
        let (a0, _) = max2[mask][0];
        let (a1, _) = max2[mask][1];
        ans[mask] = cmp::max(ans[mask], a0 + a1);
        ans[mask] = cmp::max(ans[mask], ans[mask - 1]);
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
