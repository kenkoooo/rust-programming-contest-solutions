const MOD: usize = (1e9 as usize) + 7;
const MAX_BIT: usize = 1 << 16;

fn mod_pow(x: usize, e: usize) -> usize {
    let mut cur = x;
    let mut result = 1;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % MOD;
        }
        cur = (cur * cur) % MOD;
        e >>= 1;
    }
    result
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let x = sc.usize_read();
    let y = sc.usize_read();
    let z = sc.usize_read();

    let mut dp = vec![0; MAX_BIT];
    dp[0] = 1;
    for _ in 0..n {
        let mut next = vec![0; MAX_BIT];
        for mask in 0..(MAX_BIT) {
            if dp[mask] == 0 {
                continue;
            }
            let mut v = mask2vec(mask);
            for a in 1..11 {
                v.push(a);
                if check(&v, x, y, z) {
                    let next_mask = vec2mask(&v);
                    next[next_mask] = (next[next_mask] + dp[mask]) % MOD;
                }
                v.pop();
            }
        }
        dp = next;
    }

    let mut ans = mod_pow(10, n);
    for mask in 0..MAX_BIT {
        ans = (ans + MOD - dp[mask]) % MOD;
    }
    println!("{}", ans);
}

fn check(v: &Vec<usize>, x: usize, y: usize, z: usize) -> bool {
    let mut check = vec![x, y, z];
    let mut pos = 2;
    for &v in v.iter().rev() {
        if check[pos] == v {
            if pos == 0 {
                return false;
            } else {
                pos -= 1;
            }
        } else if check[pos] < v {
            return true;
        } else {
            check[pos] -= v;
        }
    }
    return true;
}

fn vec2mask(v: &Vec<usize>) -> usize {
    let mut result = 0;
    for &a in v {
        result <<= a;
        result += (1 << (a - 1));
    }
    result & (MAX_BIT - 1)
}

fn mask2vec(mask: usize) -> Vec<usize> {
    let mut cur = mask;
    let mut result = vec![];
    let mut count = 1;
    while cur > 0 {
        if cur & 1 == 0 {
            count += 1;
        } else {
            result.push(count);
            count = 1;
        }
        cur >>= 1;
    }
    result.reverse();
    result
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
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

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
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
