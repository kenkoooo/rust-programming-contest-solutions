use std::collections::BTreeSet;

fn dp(xs: &Vec<i64>, to: i64) -> bool {
    let sum = xs.iter().sum::<i64>();
    if (sum + to) % 2 != 0 { return false; }
    let mut cur = BTreeSet::new();
    cur.insert(0);
    for &x in xs {
        let mut next = BTreeSet::new();
        for &c in cur.iter() {
            next.insert(c + x);
            next.insert(c - x);
        }
        cur = next;
    }
    return cur.contains(&to);
}

fn main() {
    let mut sc = Scanner::new();
    let s: Vec<char> = (sc.read::<String>() + "T").chars().collect();
    let mut x: i64 = sc.read();
    let y: i64 = sc.read();

    let mut sx = Vec::new();
    let mut sy = Vec::new();
    let mut to_x = true;
    let mut buf = 0;
    for &c in &s {
        if c == 'F' {
            buf += 1;
        } else {
            if to_x {
                if sx.is_empty() && sy.is_empty() {
                    x -= buf;
                } else {
                    sx.push(buf);
                }
            } else {
                sy.push(buf);
            }
            to_x = !to_x;
            buf = 0;
        }
    }

    if dp(&sx, x) && dp(&sy, y) {
        println!("Yes");
    } else {
        println!("No");
    }
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

