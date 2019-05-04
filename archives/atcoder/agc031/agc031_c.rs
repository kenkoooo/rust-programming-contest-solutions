fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a = sc.read();
    let b = sc.read();
    match solve(n, a, b) {
        Some(ans) => {
            println!("YES");
            for ans in ans.into_iter() {
                print!("{} ", ans);
            }
            println!();
        }
        None => {
            println!("NO");
        }
    }
}

fn solve(n: usize, a: i32, b: i32) -> Option<Vec<i32>> {
    if n == 0 {
        return Some(vec![a]);
    }
    let difference = (0..18).filter(|i| (1 << i) & a != (1 << i) & b).count();
    if difference & 1 == 0 {
        return None;
    }

    let k = (0..).find(|i| (1 << i) & a != (1 << i) & b).unwrap();
    assert!((1 << k) & a != (1 << k) & b);
    let mask = (1 << k) - 1;

    let new_a = ((a >> (k + 1)) << k) + (a & mask);
    let new_b = ((b >> (k + 1)) << k) + (b & mask);
    let x = new_a ^ 1;
    let prefix = if n == 1 {
        Some(vec![new_a])
    } else {
        solve(n - 1, new_a, x)
    };
    let suffix = if n == 1 {
        Some(vec![new_b])
    } else {
        solve(n - 1, x, new_b)
    };
    match (prefix, suffix) {
        (Some(prefix), Some(suffix)) => {
            let mut ans = vec![];
            for p in prefix.into_iter() {
                let prefix = (p >> k) << (k + 1);
                let suffix = p & mask;
                let v = prefix + suffix + ((1 << k) & a);
                ans.push(v);
            }
            for p in suffix.into_iter() {
                let prefix = (p >> k) << (k + 1);
                let suffix = p & mask;
                let v = prefix + suffix + ((1 << k) & b);
                ans.push(v);
            }
            Some(ans)
        }
        _ => None,
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
