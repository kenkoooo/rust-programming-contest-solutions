use std::collections::BTreeMap;

const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: u64 = sc.read();
    let mut map = BTreeMap::new();
    println!("{}", solve(n, n, &mut map));
}

fn solve(sum: u64, xor: u64, map: &mut BTreeMap<(u64, u64), u64>) -> u64 {
    if sum == 0 {
        return 1;
    }
    if let Some(&ans) = map.get(&(sum, xor)) {
        return ans;
    }
    let mut result = 0;

    // odd + odd
    if sum >= 2 {
        result += solve((sum - 2) >> 1, xor >> 1, map);
    }

    // even + even
    result += solve(sum >> 1, xor >> 1, map);

    // odd + even
    if sum > 0 {
        result += solve((sum - 1) >> 1, xor >> 1, map);
    }
    map.insert((sum, xor), result % MOD);
    result % MOD
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
