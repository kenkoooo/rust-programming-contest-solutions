use std::cmp;
use std::collections::{BTreeMap, VecDeque};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let l: usize = sc.read();
    let n = sc.read();
    let x: Vec<usize> = sc.read_vec(n);
    let y: Vec<usize> = x.iter().map(|&x| l - x).rev().collect();
    let ans = cmp::max(solve(x, l), solve(y, l));
    println!("{}", ans);
}

fn solve(x: Vec<usize>, l: usize) -> usize {
    let n = x.len();
    let mut left = vec![0; n + 1];
    for i in 0..n {
        left[i + 1] = left[i] + (l - x[n - 1 - i]) * 2;
    }
    let mut right = vec![0; n + 1];
    for i in 0..n {
        right[i + 1] = right[i] + x[i] * 2;
    }

    let mut ans = 0;
    for i in 0..n {
        let rest = n - i;
        let left_count = rest / 2;
        let right_count = rest - left_count;
        let left_sum = left[left_count];
        let right_sum = right[n - left_count] - right[i];
        let last = if right_count > left_count {
            x[n - left_count - 1]
        } else {
            l - x[n - left_count]
        };
        let sum = left_sum + right_sum - last;
        ans = cmp::max(ans, sum);
    }
    ans
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
