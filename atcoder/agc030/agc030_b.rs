use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let l: u64 = sc.read();
    let n = sc.read();
    let mut x: Vec<u64> = sc.vec(n);
    let ans1 = solve(&x, l);
    x = x.into_iter().rev().map(|x| l - x).collect();
    let ans2 = solve(&x, l);
    println!("{}", cmp::max(ans1, ans2));
}

fn solve(x: &Vec<u64>, l: u64) -> u64 {
    let n = x.len();
    let mut left_sum = vec![0; n + 1];
    for i in 0..n {
        left_sum[i + 1] = left_sum[i] + (l - x[n - 1 - i]);
    }
    let mut right_sum = vec![0; n + 1];
    for i in 0..n {
        right_sum[i + 1] = right_sum[i] + x[i];
    }

    let mut result = 0;
    for i in 0..n {
        let rest = n - i;
        let mut ans = 0;
        ans += (right_sum[rest / 2 + i] - right_sum[i]) * 2;
        ans += left_sum[rest / 2] * 2;
        if rest % 2 == 0 && rest / 2 > 0 {
            ans -= l - x[n - rest / 2];
        } else {
            ans += x[rest / 2 + i];
        }
        result = cmp::max(result, ans);
    }
    result
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
