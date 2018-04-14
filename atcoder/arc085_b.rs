use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let z: i32 = sc.read();
    let w: i32 = sc.read();
    let mut a = vec![z, w];
    for _ in 0..n {
        a.push(sc.read::<i32>());
    }

    let n = n + 2;
    let mut dp = vec![vec![vec![-1; 2]; n]; n];
    println!("{}", rec(&a, 0, 1, 0, &mut dp));
}

fn rec(cards: &Vec<i32>, xi: usize, yi: usize, turn: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    let last = cmp::max(xi, yi);
    if last == cards.len() - 1 {
        return (cards[xi] - cards[yi]).abs();
    }

    if dp[xi][yi][turn] >= 0 {
        return dp[xi][yi][turn];
    }

    if turn == 0 {
        let m1 = rec(cards, last + 1, yi, 0, dp);
        let m2 = rec(cards, last + 1, yi, 1, dp);
        dp[xi][yi][0] = cmp::max(m1, m2);
        return dp[xi][yi][0];
    } else {
        let m1 = rec(cards, xi, last + 1, 0, dp);
        let m2 = rec(cards, xi, last + 1, 1, dp);

        dp[xi][yi][1] = cmp::min(m1, m2);
        return dp[xi][yi][1];
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

