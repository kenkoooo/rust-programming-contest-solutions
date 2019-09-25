fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: u64 = sc.read();

    for h in 1..3501 {
        for w in 1..3501 {
            // 4/n - 1/h - 1/w = 4*h*w - w*n - h*n
            if 4 * h * w < w * n + h * n {
                continue;
            }
            let a = 4 * h * w - w * n - h * n;
            let b = h * n * w;
            if a > 0 && b % a == 0 {
                println!("{} {} {}", b / a, w, h);
                return;
            }
        }
    }
    unreachable!()
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
