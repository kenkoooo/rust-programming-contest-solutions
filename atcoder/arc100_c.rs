use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(1 << n);
    let mut top2 = vec![vec![]; 1 << n];
    for mask in 0..(1 << n) {
        top2[mask].push((a[mask], mask));
    }
    for mask in 0..(1 << n) {
        for i in 0..n {
            let next_mask = mask | (1 << i);
            let top2_mask = top2[mask].clone();
            top2[next_mask].extend(top2_mask);
            top2[next_mask].sort();
            top2[next_mask].dedup();
            top2[next_mask].reverse();
            if top2[next_mask].len() > 2 {
                top2[next_mask].resize(2, (0, 0));
            }
        }
    }

    let mut max = 0;
    for mask in 1..(1 << n) {
        let (a, _) = top2[mask][0];
        let (b, _) = top2[mask][1];
        max = cmp::max(max, a + b);
        println!("{}", max);
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
