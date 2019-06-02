use std::collections::{BTreeMap, BTreeSet};
const MOD: usize = 1e9 as usize + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: i64 = sc.read();
    let mut memo = BTreeMap::new();
    println!("{}", solve(n, n, &mut memo));
}

fn naive(n: i64) -> usize {
    let mut set = BTreeSet::new();

    for a in 0..(n + 1) {
        for b in 0..(n + 1) {
            let u = a ^ b;
            let v = a + b;
            if u <= n && v <= n {
                set.insert((u, v));
            }
        }
    }
    println!("{:?}", set);
    set.len()
}

fn solve(sum: i64, xor: i64, memo: &mut BTreeMap<(i64, i64), usize>) -> usize {
    match memo.get(&(sum, xor)) {
        Some(&ans) => ans,
        None => {
            if sum == 0 {
                return 1;
            }
            let mut result = 0;

            // odd & odd
            if sum >= 2 {
                result += solve((sum - 2) >> 1, xor >> 1, memo);
            }

            // odd & even
            result += solve((sum - 1) >> 1, xor >> 1, memo);

            // even & even
            result += solve(sum >> 1, xor >> 1, memo);

            result %= MOD;
            memo.insert((sum, xor), result);
            result
        }
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
