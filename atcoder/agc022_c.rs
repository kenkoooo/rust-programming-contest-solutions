use std::cmp;
use std::collections::BTreeSet;


const MAX_D: usize = 50;
const MAX_COST: usize = 1 << 55;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<usize> = (0..n).map(|_| sc.read()).collect();
    let b: Vec<usize> = (0..n).map(|_| sc.read()).collect();

    let mut ans: Vec<usize> = Vec::new();
    for upper in (1..(MAX_D + 2)).rev() {
        let mut costs = vec![vec![MAX_COST; MAX_D + 1]; MAX_D + 1];

        for d in 1..upper {
            for from in 0..(MAX_D + 1) {
                let to = from % d;
                costs[from][to] = (1 << d);
            }
        }

        for d in &ans {
            let d = *d;
            for from in 0..(MAX_D + 1) {
                let to = from % d;
                costs[from][to] = (1 << d);
            }
        }

        for i in 0..(MAX_D + 1) {
            costs[i][i] = 0;
        }

        for k in 0..(MAX_D + 1) {
            for i in 0..(MAX_D + 1) {
                for j in 0..(MAX_D + 1) {
                    costs[i][j] = cmp::min(costs[i][j], costs[i][k] + costs[k][j]);
                }
            }
        }

        let mut check = true;
        for i in 0..n {
            if costs[a[i]][b[i]] == MAX_COST {
                check = false;
            }
        }
        if !check {
            if upper == MAX_D + 1 {
                println!("-1");
                return;
            }
            ans.push(upper);
        }
    }

    let mut cost: usize = 0;
    for a in &ans {
        cost += (1 << *a);
    }
    println!("{}", cost);
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

