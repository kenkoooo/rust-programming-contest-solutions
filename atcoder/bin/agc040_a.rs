use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc.chars();
    let n = s.len() + 1;

    let mut x = vec![];
    for i in 1..(n - 1) {
        let left = s[i - 1];
        let right = s[i];
        if left == '<' && right == '>' {
            x.push(i);
        }
    }

    if s[0] == '>' {
        x.push(0);
    }
    if s[n - 2] == '<' {
        x.push(n - 1);
    }

    let mut ans = vec![0; n];
    for x in x.into_iter() {
        if x == 0 {
            let mut right = x;
            while right + 1 < n - 1 && s[right + 1] == '>' {
                right += 1;
            }
            for i in x..(right + 2) {
                ans[i] = cmp::max(ans[i], right + 1 - i);
            }
        } else if x == n - 1 {
            let mut left = x - 1;
            while left > 0 && s[left - 1] == '<' {
                left -= 1;
            }
            for i in left..(x + 1) {
                ans[i] = cmp::max(i - left, ans[i]);
            }
        } else {
            let mut left = x - 1;
            while left > 0 && s[left - 1] == '<' {
                left -= 1;
            }
            let mut right = x;
            while right + 1 < n - 1 && s[right + 1] == '>' {
                right += 1;
            }
            for i in left..(x + 1) {
                ans[i] = cmp::max(i - left, ans[i]);
            }
            for i in x..(right + 2) {
                ans[i] = cmp::max(ans[i], right + 1 - i);
            }
        }
    }

    println!("{}", ans.iter().sum::<usize>());
}

/// < >>> << >0<<<<<5>>> <

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
