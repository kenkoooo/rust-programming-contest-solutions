use std::cmp;

const INF: u32 = 1e7 as u32;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc
        .chars()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let n = s.len();

    let mut min = vec![INF; 1 << 26];
    min[0] = 0;
    let mut hash = 0;
    for i in 0..n {
        hash ^= 1 << s[i];
        let split_min = (0..26)
            .map(|i| {
                let palindrome_prefix = 1 << i;
                let suffix = hash ^ palindrome_prefix;
                min[suffix] + 1
            })
            .min()
            .unwrap();

        let dp = cmp::min(split_min, min[hash] + 1);
        min[hash] = cmp::min(min[hash], dp);
        if i == n - 1 {
            println!("{}", dp);
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
