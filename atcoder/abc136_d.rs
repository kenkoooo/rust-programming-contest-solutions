fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc.chars();
    let n = s.len();

    let mut candidates = vec![];
    for i in 0..(n - 1) {
        if s[i] == 'R' && s[i + 1] == 'L' {
            candidates.push(i);
        }
    }

    let x = candidates.len();
    let mut ans = vec![0; n];
    for i in 0..x {
        let mut left = candidates[i];
        let mut right = candidates[i] + 1;
        while left > 0 && s[left - 1] == 'R' {
            left -= 1;
        }
        while right + 1 < n && s[right + 1] == 'L' {
            right += 1;
        }
        let mut odd = 0;
        let mut even = 0;
        let i = candidates[i];
        for j in left..i {
            if (i - j) % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        for j in i..(right + 1) {
            if (j - i) % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        ans[i] += even;
        ans[i + 1] += odd;
    }
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
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
