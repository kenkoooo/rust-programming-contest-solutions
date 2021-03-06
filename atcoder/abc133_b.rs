fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let d: usize = sc.read();
    let x: Vec<Vec<i64>> = (0..n).map(|_| sc.vec(d)).collect();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            let mut dist2 = 0;
            for k in 0..d {
                let dx = x[i][k] - x[j][k];
                dist2 += dx * dx;
            }

            let sq = (dist2 as f64).sqrt() as i64;
            if sq * sq == dist2 {
                ans += 1;
            }
        }
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
