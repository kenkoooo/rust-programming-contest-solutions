use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut xy: Vec<(u64, u64)> = (0..n).map(|_| (sc.read(), sc.read())).collect();
    let max = xy.iter().map(|&(x, y)| cmp::max(x, y)).max().unwrap();
    let min = xy.iter().map(|&(x, y)| cmp::min(x, y)).min().unwrap();
    let max_min = xy.iter().map(|&(x, y)| cmp::max(x, y)).min().unwrap();
    let min_max = xy.iter().map(|&(x, y)| cmp::min(x, y)).max().unwrap();
    let ans1 = (max - max_min) * (min_max - min);

    xy = xy
        .into_iter()
        .map(|(x, y)| (cmp::min(x, y), cmp::max(x, y)))
        .collect();
    xy.sort();
    let mut ans2 = max - min;
    let mut right_min = max;
    let mut max2 = xy[n - 1].0;
    for i in 0..(n - 1) {
        let (_, right) = xy[i];
        right_min = cmp::min(right_min, right);
        max2 = cmp::max(max2, right);
        let min = cmp::min(xy[i + 1].0, right_min);
        ans2 = cmp::min(ans2, max2 - min);
    }

    let ans2 = ans2 * (max - min);
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
