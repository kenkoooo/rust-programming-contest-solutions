use std::collections::BTreeMap;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();

    let mut dp: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    println!("{}", rec(n, n, &mut dp));
}

fn rec(xor: usize, sum: usize, dp: &mut BTreeMap<(usize, usize), usize>) -> usize {
    if sum == 0 { return 1; }
    if let Some(&ans) = dp.get(&(xor, sum)) {
        return ans;
    }

    // odd & odd
    let mut result = if sum >= 2 { rec(xor >> 1, (sum - 2) >> 1, dp) } else { 0 };

    // odd & even
    result += rec((xor - 1) >> 1, (sum - 1) >> 1, dp);

    // even & even
    result += rec(xor >> 1, sum >> 1, dp);

    result %= MOD;
    dp.insert((xor, sum), result);
    return result;
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
