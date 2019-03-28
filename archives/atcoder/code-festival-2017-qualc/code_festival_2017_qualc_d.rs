use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s: Vec<_> = sc
        .chars()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect();
    let n = s.len();

    let mut hash: usize = 0;
    let mut min: Vec<u32> = vec![n as u32; 1 << 26];
    for i in 0..n {
        hash ^= 1 << s[i];
        let mut ans = n as u32;
        if hash.count_ones() <= 1 {
            ans = 1;
        } else {
            for j in 0..26 {
                ans = cmp::min(ans, min[hash ^ (1 << j)] + 1);
            }
            ans = cmp::min(ans, min[hash]);
        }
        min[hash] = cmp::min(min[hash], ans);
        if i == n - 1 {
            println!("{}", ans);
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
