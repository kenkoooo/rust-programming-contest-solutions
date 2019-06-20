fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a: u64 = sc.read();
    let b: u64 = sc.read();
    match solve(a, b, n) {
        Some(ans) => {
            println!("YES");
            for (i, ans) in ans.into_iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", ans);
            }
            println!();
        }
        None => {
            println!("NO");
        }
    }
}

fn solve(a: u64, b: u64, n: usize) -> Option<Vec<u64>> {
    if (a ^ b).count_ones() & 1 == 0 {
        return None;
    }
    if n == 1 {
        return Some(vec![a, b]);
    }
    let k = (0..).find(|&i| (a & (1 << i)) != (b & (1 << i))).unwrap();
    let a_bit = (a >> k) & 1;
    let b_bit = (b >> k) & 1;
    assert_ne!(a_bit, b_bit);

    let next_a = remove_bit(a, k);
    let next_b = remove_bit(b, k);
    let next_c = next_a ^ 1;

    let prefix = solve(next_a, next_c, n - 1);
    let suffix = solve(next_c, next_b, n - 1);
    match (prefix, suffix) {
        (Some(prefix), Some(suffix)) => {
            let mut result = vec![];
            for p in prefix.into_iter() {
                result.push(insert(p, k, a_bit));
            }
            for s in suffix.into_iter() {
                result.push(insert(s, k, b_bit));
            }
            Some(result)
        }
        _ => None,
    }
}

fn remove_bit(x: u64, i: u64) -> u64 {
    let prefix = (x >> (i + 1)) << i;
    let suffix = x & ((1 << i) - 1);
    assert_eq!(prefix & suffix, 0);
    prefix | suffix
}

fn insert(x: u64, i: u64, b: u64) -> u64 {
    let suffix = x & ((1 << i) - 1);
    let prefix = (x >> i) << (i + 1);
    assert_eq!(prefix & (1 << i), 0);
    assert_eq!(suffix & (1 << i), 0);
    assert_eq!(prefix & suffix, 0);
    prefix | (b << i) | suffix
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
