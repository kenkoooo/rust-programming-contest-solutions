use std::cmp;

const INF: i64 = 1e16 as i64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let a = sc.read();
    let b = sc.read();
    let q = sc.read();
    let s: Vec<i64> = sc.vec(a);
    let t: Vec<i64> = sc.vec(b);
    let x: Vec<i64> = sc.vec(q);

    let shrine = q + 1;
    let temple = q + 2;

    let mut all = vec![];
    all.extend(s.iter().map(|&s| (s, shrine)));
    all.extend(t.iter().map(|&t| (t, temple)));
    all.extend(x.iter().enumerate().map(|(i, &x)| (x, i)));
    all.sort();

    let mut prev_temple = vec![0; q];
    let mut prev_shrine = vec![0; q];
    let mut last_shrine = -INF;
    let mut last_temple = -INF;
    for &(x, i) in all.iter() {
        if i == shrine {
            last_shrine = x;
        } else if i == temple {
            last_temple = x;
        } else {
            prev_shrine[i] = last_shrine;
            prev_temple[i] = last_temple;
        }
    }

    let mut next_temple = vec![0; q];
    let mut next_shrine = vec![0; q];
    let mut last_shrine = INF;
    let mut last_temple = INF;
    for &(x, i) in all.iter().rev() {
        if i == shrine {
            last_shrine = x;
        } else if i == temple {
            last_temple = x;
        } else {
            next_shrine[i] = last_shrine;
            next_temple[i] = last_temple;
        }
    }

    for i in 0..q {
        let next_shrine = next_shrine[i] - x[i];
        let next_temple = next_temple[i] - x[i];
        let prev_shrine = x[i] - prev_shrine[i];
        let prev_temple = x[i] - prev_temple[i];

        let prev = cmp::max(prev_shrine, prev_temple);
        let next = cmp::max(next_shrine, next_temple);

        let c1 = cmp::min(prev_shrine * 2 + next_temple, prev_shrine + next_temple * 2);
        let c2 = cmp::min(next_shrine * 2 + prev_temple, next_shrine + prev_temple * 2);

        println!("{}", cmp::min(cmp::min(c1, c2), cmp::min(prev, next)));
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
