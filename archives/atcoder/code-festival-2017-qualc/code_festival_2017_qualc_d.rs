use std::cmp;

const INF: u32 = 1 << 30 as u32;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let s: Vec<u32> = sc
        .read::<String>()
        .chars()
        .map(|c| c as u32 - 'a' as u32)
        .collect();
    let n = s.len();
    let mut hash: Vec<u32> = vec![0; n + 1];
    for i in 0..n {
        hash[i + 1] = hash[i] ^ (1 << s[i]);
    }

    let mut opt = vec![INF; n + 1];
    let mut min = vec![INF; 1 << 26];
    opt[0] = 0;
    min[0] = 0;

    for i in 0..n {
        let hash = hash[i + 1] as usize;
        for x in 0..27 {
            let prefix = if x == 0 { 0 } else { 1 << (x - 1) };
            let suffix = hash ^ prefix;
            opt[i + 1] = cmp::min(opt[i + 1], min[suffix] + 1);
        }
        min[hash] = cmp::min(min[hash], opt[i + 1]);
    }

    println!("{}", opt[n]);
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
