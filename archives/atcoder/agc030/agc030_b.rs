use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let l: u64 = sc.read();
    let n: usize = sc.read();
    let x: Vec<u64> = sc.vec(n);

    let ans1 = solve(&x, l);
    let x = x.into_iter().rev().map(|x| l - x).collect();
    let ans2 = solve(&x, l);

    println!("{}", cmp::max(ans1, ans2));
}

fn solve(x: &Vec<u64>, l: u64) -> u64 {
    let n = x.len();

    let mut left_sum = vec![0; n + 1];
    let mut right_sum = vec![0; n + 1];
    for i in 0..n {
        right_sum[i + 1] = right_sum[i] + x[i];
        left_sum[i + 1] = left_sum[i] + (l - x[n - 1 - i]);
    }

    let mut ans = 0;
    for i in 0..n {
        let rest = n - (i + 1);
        let left = (rest + 1) / 2;
        let right = n - left;
        let right_sum = right_sum[right] - right_sum[i + 1];
        let dist = left_sum[left] * 2 + right_sum * 2 + x[i] * 2;
        let last = if rest % 2 == 0 {
            x[right - 1]
        } else {
            l - x[right]
        };

        ans = cmp::max(ans, dist - last);
    }

    ans
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
