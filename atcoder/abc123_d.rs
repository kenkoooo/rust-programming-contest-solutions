use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let x: usize = sc.read();
    let y: usize = sc.read();
    let z: usize = sc.read();
    let k: usize = sc.read();
    let a: Vec<i64> = sc.vec(x);
    let b: Vec<i64> = sc.vec(y);
    let c: Vec<i64> = sc.vec(z);
    let mut ab = vec![];
    for i in 0..x {
        for j in 0..y {
            ab.push(a[i] + b[j]);
        }
    }
    ab.sort();
    ab.reverse();

    let mut ans = vec![];
    for &ab in (&ab[..cmp::min(k, x * y)]).iter() {
        for i in 0..z {
            ans.push(ab + c[i]);
        }
    }
    ans.sort();
    ans.reverse();
    for i in 0..k {
        println!("{}", ans[i]);
    }
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
