use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: usize = sc.read();
    let b: usize = sc.read();
    if a + b > n + 1 {
        println!("-1");
        return;
    }

    let mut ans = (1..(n + 1)).collect::<Vec<usize>>();
    for i in 0..((n + b - 1) / b) {
        ans[(i * b)..cmp::min(n, i * b + b)].reverse();
    }

    let mut cur = (n + b - 1) / b;
    if cur > a {
        println!("-1");
        return;
    }
    for i in (0..((n + b - 1) / b)).rev() {
        let from = (i * b);
        let to = cmp::min(n, i * b + b);
        let element = to - from;
        let reversing = cmp::min(element, a - cur + 1);
        ans[from..(reversing + from)].reverse();
        cur += reversing - 1;
        if a == cur {
            break;
        }
    }

    for x in ans.into_iter() {
        print!("{} ", x);
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
