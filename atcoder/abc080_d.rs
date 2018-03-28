use std::cmp;

const MAX_C: usize = 30;
const MAX_N: usize = 100000;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let c: usize = sc.read();

    let mut records: Vec<Vec<(usize, usize)>> = (0..MAX_C).map(|_| Vec::new()).collect();
    for _ in 0..n {
        let s = sc.read::<usize>() - 1;
        let t = sc.read::<usize>() - 1;
        let c = sc.read::<usize>() - 1;
        records[c].push((s, t));
    }

    let mut starts = vec![0; MAX_N];
    let mut ends = vec![0; MAX_N];
    for c in 0..MAX_C {
        records[c].sort();
        for i in 0..records[c].len() {
            if i == 0 {
                let (s, t) = records[c][i];
                starts[s] += 1;
                ends[t] += 1;
            } else {
                let (ps, pt) = records[c][i - 1];
                let (s, t) = records[c][i];
                starts[s] += 1;
                ends[t] += 1;
                if pt == s {
                    ends[s] -= 1;
                    starts[s] -= 1;
                }
            }
        }
    }

    let mut ans = 0;
    let mut cur = 0;
    for i in 0..MAX_N {
        cur += starts[i];
        ans = cmp::max(ans, cur);
        cur -= ends[i];
    }

    println!("{}", ans);
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}

