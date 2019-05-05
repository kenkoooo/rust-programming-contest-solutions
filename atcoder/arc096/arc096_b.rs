use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let c: i64 = sc.read();
    let xv: Vec<(i64, i64)> = (0..n).map(|_| (sc.read(), sc.read())).collect();

    let mut right_max = vec![0; n + 1];
    let mut right_back = vec![0; n + 1];
    let mut right_v = 0;
    for i in 0..n {
        let (x, v) = xv[i];
        right_v += v;
        right_max[i + 1] = cmp::max(right_max[i], right_v - x);
        right_back[i + 1] = cmp::max(right_back[i], right_v - 2 * x);
    }

    let mut left_max = vec![0; n + 1];
    let mut left_back = vec![0; n + 1];
    let mut left_v = 0;
    for i in 0..n {
        let (x, v) = xv[n - 1 - i];
        left_v += v;
        left_max[i + 1] = cmp::max(left_max[i], left_v - (c - x));
        left_back[i + 1] = cmp::max(left_back[i + 1], left_v - 2 * (c - x));
    }

    let mut ans = 0;
    for i in 0..n {
        ans = cmp::max(ans, left_max[i + 1]);
        ans = cmp::max(ans, right_max[i + 1]);

        ans = cmp::max(ans, right_back[i + 1] + left_max[n - 1 - i]);
        ans = cmp::max(ans, left_back[i + 1] + right_max[n - 1 - i]);
    }

    println!("{}", ans);
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
