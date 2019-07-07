use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let x: u64 = sc.read();
    let v: Vec<u64> = sc.vec(n);

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + v[i];
    }

    let mut min = 1e18 as u64;
    for k in 1..(n + 1) {
        let mut ans = x * ((n + k) as u64);
        for a in 0.. {
            if n < a * k {
                break;
            }
            let sum = if n >= (a + 1) * k {
                sum[n - a * k] - sum[n - (a + 1) * k]
            } else {
                sum[n - a * k]
            };
            let m = if a == 0 { 5 } else { 2 * (a + 2) - 1 };
            ans += sum * (m as u64);
        }
        min = cmp::min(min, ans);
    }
    println!("{}", min);
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
