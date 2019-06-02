use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: usize = sc.read();
    let b: usize = sc.read();
    if a + b > n + 1 || (n + b - 1) / b > a {
        println!("-1");
        return;
    }

    let mut ans = (1..(n + 1)).collect::<Vec<_>>();
    let segments = (n + b - 1) / b;
    let mut remain = n - a;
    for i in 0..segments {
        let from = i * b;
        let to = cmp::min(from + cmp::min(b, remain + 1), n);
        ans[from..to].reverse();
        remain -= to - from - 1;
        if remain == 0 {
            break;
        }
    }

    assert_eq!(remain, 0);
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
