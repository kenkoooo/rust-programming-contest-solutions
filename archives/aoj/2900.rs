use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let a: Vec<i64> = sc.vec(n);
    let ans1 = solve(a.clone());
    let ans2 = solve(a.into_iter().map(|a| -a).collect());

    println!("{}", cmp::min(ans1, ans2));
}

fn solve(mut a: Vec<i64>) -> usize {
    let n = a.len();
    let mut ans = 0;
    for i in 1..n {
        if i % 2 == 0 {
            a[i - 1] *= -1;
            a[i] *= -1;
            if i + 1 < n {
                a[i + 1] *= -1;
            }
        }

        if a[i - 1] < a[i] {
            if i + 1 < n && a[i + 1] < a[i - 1] {
                a.swap(i, i + 1);
            } else {
                a.swap(i - 1, i);
            }
            ans += 1;
        }

        if i % 2 == 0 {
            a[i - 1] *= -1;
            a[i] *= -1;
            if i + 1 < n {
                a[i + 1] *= -1;
            }
        }
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
