use std::cmp;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let d: usize = sc.read();
    let x: Vec<usize> = sc.read_vec(n);

    let mut right_walk = vec![n; n];
    let mut right = 0;
    for i in 0..n {
        right = cmp::max(right, i);
        while right + 1 < n && x[right + 1] - x[i] <= d {
            right += 1;
        }
        right_walk[i] = right;
    }

    let mut left = n - 1;
    let mut left_walk = vec![n; n];
    for i in (0..n).rev() {
        left = cmp::min(left, i);
        while left > 0 && x[i] - x[left - 1] <= d {
            left -= 1;
        }
        left_walk[i] = left;
    }

    // println!("{:?}", right_walk);
    // println!("{:?}", left_walk);

    let mut q = VecDeque::new();
    q.push_front(0);
    let mut max_tail = 0;
    let mut cur = 0;
    let mut ans = 0;
    for j in 1..n {
        while !q.is_empty() && x[j] - x[head(&mut q)] > d {
            let h = q.pop_front().unwrap();
            if !q.is_empty() {
                let t = tail(&mut q);
                let max = right_walk[h];
                if t > max {
                    // println!("{:?} t={} max={} cur={} h={}", q, t, max, cur, h);
                    cur -= t - max;
                }
            }
        }

        while max_tail < j || (max_tail + 1 < n && x[max_tail + 1] - x[j] <= d) {
            max_tail += 1;
            q.push_back(max_tail);
            let min = left_walk[max_tail];
            let h = head(&mut q);
            if min > h {
                cur += min - h;
            }
        }
        ans += cur;
    }

    println!("{}", ans);
}

fn head(q: &mut VecDeque<usize>) -> usize {
    let t = q.pop_front().unwrap();
    q.push_front(t);
    t
}

fn tail(q: &mut VecDeque<usize>) -> usize {
    let t = q.pop_back().unwrap();
    q.push_back(t);
    t
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
