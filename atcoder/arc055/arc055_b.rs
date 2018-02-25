fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let k = sc.read::<usize>();

    let mut dp = vec![vec![-1.0; n + 1]; n + 1];
    for i in 0..(n + 1) {
        dp[i][i] = (i as f64) / (n as f64);
        dp[i][0] = 0.0;
    }
    println!("{}", rec(n, k, &mut dp, n));
}

/// take `take` cookies from `rest` cookies.
fn rec(rest: usize, take: usize, dp: &mut Vec<Vec<f64>>, total: usize) -> f64 {
    if dp[rest][take] >= 0.0 {
        return dp[rest][take];
    }

    let turn = total + 1 - rest;

    // probability to get current max in this turn
    let get_current_max_probability = 1.0 / turn as f64;
    let skip = rec(rest - 1, take, dp, total);

    let get_global_maximum_by_eat = 1.0 / (total as f64) // probability to get global maximum in this turn
        + get_current_max_probability * rec(rest - 1, take - 1, dp, total) // probability to get global maximum after this turn
        + (1.0 - get_current_max_probability) * skip; // if current maximum can't be gained in this turn, this turn should be skipped.

    dp[rest][take] = if get_global_maximum_by_eat > skip { get_global_maximum_by_eat } else { skip };
    return dp[rest][take];
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}

