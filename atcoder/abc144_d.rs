fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let a: f64 = sc.read();
    let b: f64 = sc.read();
    let x: f64 = sc.read();
    let s = x / a;
    // b*z/2==s
    let z = s * 2.0 / b;
    if z > a {
        let s = a * b - s;
        let z = s * 2.0 / a;
        // tan(t) = z/a
        let t = z.atan2(a);
        let d = t / std::f64::consts::PI * 180.0;
        //        eprintln!("z={} t={}", z, t);
        println!("{}", d);
    } else {
        // tan(t) = b/z
        let t = b.atan2(z);
        let d = t / std::f64::consts::PI * 180.0;
        //        eprintln!("z={} t={}", z, t);
        println!("{}", d);
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
