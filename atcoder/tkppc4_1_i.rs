const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let m = sc.read();
    let mut a: Vec<u64> = sc.vec(n);
    let mut b: Vec<u64> = sc.vec(m);
    a.sort();
    b.sort();

    let mut r_candidates = vec![0; n];
    let mut b_pos = 0;
    for i in 0..n {
        while b_pos < m && b[b_pos] < a[i] {
            b_pos += 1;
        }
        assert!(b_pos == m || b[b_pos] >= a[i]);
        r_candidates[i] = b_pos;
    }

    let mut s_candidates = vec![0; n];
    let mut b_pos = 0;
    for i in (0..n).rev() {
        while b_pos < m && a[i] < b[m - 1 - b_pos] {
            b_pos += 1;
        }
        assert!(b_pos == m || b[m - 1 - b_pos] <= a[i]);
        s_candidates[i] = b_pos;
    }

    let mut r_sum = r_candidates[0];
    let mut ans = 0;
    for q in 1..n {
        ans += r_sum * s_candidates[q];
        ans %= MOD;
        r_sum += r_candidates[q];
        r_sum %= MOD;
    }
    println!("{}", ans);
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
