fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut ans = vec![vec![0; n]; n];
    let mut cur = 1;
    construct(&mut ans, 1, 0, n);
    for i in 0..n {
        for j in (i + 1)..n {
            print!("{} ", ans[i][j]);
        }
        println!();
    }
}

fn construct(ans: &mut Vec<Vec<u64>>, depth: u64, from: usize, to: usize) {
    let length = to - from;
    let prefix = length / 2;
    let suffix = length - prefix;
    for i in 0..prefix {
        for j in 0..suffix {
            let left = from + i;
            let right = from + prefix + j;
            ans[left][right] = depth;
            ans[right][left] = depth;
        }
    }
    if prefix >= 2 {
        construct(ans, depth + 1, from, from + prefix);
    }
    if suffix >= 2 {
        construct(ans, depth + 1, from + prefix, to);
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
