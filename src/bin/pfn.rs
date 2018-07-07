const MOD: u32 = 1_000_000_007;

fn solve_fast(s: &Vec<char>) -> u32 {
    let n = s.len();
    let mut count = vec![0; 26];
    for &s in s {
        count[((s as u8) - ('a' as u8)) as usize] += 1;
    }
    let mut s = vec![];
    let mut m = 0;
    for &c in &count {
        if c == 0 {
            continue;
        }
        for _ in 0..c {
            s.push(m);
        }
        m += 1;
    }

    let mut dp = vec![vec![0; m + 1]; 1 << n];
    dp[0][m] = 1;
    for mask in 0..(1 << n) {
        for kind in 0..(m + 1) {
            if dp[mask][kind] == 0 {
                continue;
            }
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    continue;
                }
                if i > 0 && mask & (1 << (i - 1)) == 0 && s[i] == s[i - 1] {
                    continue;
                }
                if kind == s[i] {
                    continue;
                }
                dp[mask | (1 << i)][s[i]] += dp[mask][kind];
                if dp[mask | (1 << i)][s[i]] > MOD {
                    dp[mask | (1 << i)][s[i]] -= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..m {
        ans = (ans + dp[(1 << n) - 1][i]) % MOD;
    }
    ans
}
fn main() {
    let mut sc = Scanner::new();
    let mut sc = Scanner::new();
    let s: Vec<char> = sc.read::<String>().chars().collect();
    println!("{}", solve_fast(&s));
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
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

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
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
