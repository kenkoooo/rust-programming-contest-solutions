use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut pairs = vec![];
    for _ in 0..n {
        let x: u64 = sc.read();
        let y: u64 = sc.read();
        let (min, max) = if x < y { (x, y) } else { (y, x) };
        pairs.push((min, max));
    }
    pairs.sort();

    let total_min = pairs[0].0;
    let total_max = pairs.iter().map(|p| p.1).max().unwrap();

    let left_max = pairs.iter().map(|p| p.0).max().unwrap();
    let right_min = pairs.iter().map(|p| p.1).min().unwrap();

    let ans1 = (left_max - total_min) * (total_max - right_min);

    let mut store_min = std::u64::MAX;
    let mut cur_max = left_max;
    let mut cur = cur_max - total_min;
    for i in 0..(n - 1) {
        let (_, max) = pairs[i];
        cur_max = cmp::max(max, cur_max);
        store_min = cmp::min(max, store_min);
        let next = pairs[i + 1].0;
        cur = cmp::min(cur, cur_max - cmp::min(store_min, next));
    }
    let ans2 = (total_max - total_min) * cur;
    println!("{}", cmp::min(ans1, ans2));
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
