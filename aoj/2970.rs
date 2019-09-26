fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let p: Vec<usize> = (0..n).map(|_| sc.read::<usize>() - 1).collect();
    let q: Vec<usize> = (0..n).map(|_| sc.read::<usize>() - 1).collect();
    let mut b = vec![0; n];
    let mut m = vec![0; n];
    for i in 0..n {
        let mut cur = p[i];
        let mut sorted: Option<i64> = None;
        for d in 0..(2 * n as i64 + 2) {
            if cur == i {
                match sorted {
                    Some(x) => {
                        b[i] = x;
                        m[i] = d - x;
                        break;
                    }
                    None => sorted = Some(d),
                }
            }
            cur = q[cur];
        }
        if m[i] == 0 {
            println!("-1");
            return;
        }
    }

    let (b, m) = chinese_remainder_theorem(&b, &m);
    if m == -1 {
        println!("-1");
    } else {
        println!("{}", b);
    }
}

fn extended_gcd(a: i64, b: i64, p: &mut i64, q: &mut i64) -> i64 {
    if b == 0 {
        *p = 1;
        *q = 0;
        a
    } else {
        let d = extended_gcd(b, a % b, q, p);
        *q -= a / b * *p;
        d
    }
}

fn chinese_remainder_theorem(b: &Vec<i64>, modulo: &Vec<i64>) -> (i64, i64) {
    let (mut r, mut m) = (0, 1);
    for i in 0..b.len() {
        let (mut p, mut q) = (0, 0);
        let d = extended_gcd(m, modulo[i], &mut p, &mut q);
        if (b[i] - r) % d != 0 {
            return (0, -1);
        }
        let tmp = ((b[i] - r) / d * p) % (modulo[i] / d);
        r += m * tmp;
        m *= modulo[i] / d;
    }
    ((r % m + m) % m, m)
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
