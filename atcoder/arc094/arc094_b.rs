use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let q: usize = sc.read();
    for _ in 0..q {
        let a: usize = sc.read();
        let b: usize = sc.read();
        let (a, b) = if a > b { (b, a) } else { (a, b) };

        let mut ok = a - 1;
        let mut ng = 1e10 as usize;
        while ng - ok > 1 {
            let x = (ok + ng) / 2;
            let mut max = 0;
            if x >= b - 1 {
                // 1 ... a-1    a+1   ... x-b+1 x-b+2 ... x
                // x ... x-a+2  x-a+1 ... b+1   b-1   ... 1
                max = get_max(1, a - 1, x, x + 2 - a);
                max = cmp::max(max, get_max(a + 1, x - (b - 1), x - (a - 1), b + 1));
                max = cmp::max(max, get_max(x + 2 - b, x, b - 1, 1));
            } else {
                // 1 ... a-1      a+1 ... x
                // x-1 ... x-a+1  x-a ... 1
                max = get_max(1, a - 1, x - 1, x - (a - 1));
                max = cmp::max(max, get_max(a + 1, x, x - a, 1));
            }

            if max < a * b {
                ok = x;
            } else {
                ng = x;
            }
        }
        println!("{}", if a <= ok { ok - 1 } else { ok });
    }
}

fn get_max(from_a: usize, to_a: usize, from_b: usize, to_b: usize) -> usize {
    if from_a > to_a {
        return 0;
    }
    assert_eq!(to_a + 1 - from_a, from_b + 1 - to_b);
    let mut m = (from_a + from_b) / 2;
    let mut max = cmp::max(from_a * from_b, to_a * to_b);
    if from_a <= m && m <= to_a {
        let da = m - from_a;
        let b = from_b - da;
        max = cmp::max(max, m * b);
    }
    m += 1;
    if from_a <= m && m <= to_a {
        let da = m - from_a;
        let b = from_b - da;
        max = cmp::max(max, m * b);
    }
    max
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
