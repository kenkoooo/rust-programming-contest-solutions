fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let m = sc.usize_read();
    let mut intervals: Vec<(usize, usize, usize)> = (0..n)
        .map(|_| {
            let l = sc.usize_read();
            let r = sc.usize_read();
            let d = r - l + 1;
            (d, l, r)
        })
        .collect();
    intervals.sort();

    let mut seg = RangeAddSegmentTree::new();
    for i in 0..(1 + m) {
        seg.update(i, 0);
    }

    let mut head = 0;
    for d in 1..(m + 1) {
        while head < n {
            let (id, l, r) = intervals[head];
            if id < d {
                seg.add(l, r + 1, 1);
                head += 1;
            } else {
                break;
            }
        }

        let mut ans = (n - head) as i64;
        let mut cur = 0;
        while cur <= m {
            ans += seg.get_min(cur, cur + 1);
            cur += d;
        }

        println!("{}", ans);
    }
}
use std::cmp;
const NUM: usize = 1 << 20;
const INF: i64 = 1 << 60;

pub struct RangeAddSegmentTree {
    seg_min: Vec<i64>,
    seg_max: Vec<i64>,
    seg_add: Vec<i64>,
}

impl RangeAddSegmentTree {
    pub fn new() -> Self {
        RangeAddSegmentTree {
            seg_min: vec![INF; NUM * 2],
            seg_max: vec![-INF; NUM * 2],
            seg_add: vec![0; NUM * 2],
        }
    }

    /// add to [a, b)
    pub fn add(&mut self, a: usize, b: usize, value: i64) {
        self.add_to_range(a, b, value, 0, 0, NUM);
    }

    fn add_to_range(
        &mut self,
        a: usize,
        b: usize,
        value: i64,
        k: usize,
        left: usize,
        right: usize,
    ) {
        if b <= left || right <= a {
            return;
        }
        if a <= left && right <= b {
            let mut k = k;
            self.seg_add[k] += value;
            while k > 0 {
                k = (k - 1) / 2;
                self.seg_min[k] = cmp::min(
                    self.seg_min[k * 2 + 1] + self.seg_add[k * 2 + 1],
                    self.seg_min[k * 2 + 2] + self.seg_add[k * 2 + 2],
                );
                self.seg_max[k] = cmp::max(
                    self.seg_max[k * 2 + 1] + self.seg_add[k * 2 + 1],
                    self.seg_max[k * 2 + 2] + self.seg_add[k * 2 + 2],
                );
            }
        } else {
            self.add_to_range(a, b, value, k * 2 + 1, left, (left + right) / 2);
            self.add_to_range(a, b, value, k * 2 + 2, (left + right) / 2, right);
        }
    }

    pub fn update(&mut self, pos: usize, value: i64) {
        let mut k = pos + NUM - 1;
        self.seg_min[k] = value;
        self.seg_max[k] = value;
        while k > 0 {
            k = (k - 1) / 2;
            self.seg_min[k] = cmp::min(self.seg_min[k * 2 + 1], self.seg_min[k * 2 + 2]);
            self.seg_max[k] = cmp::max(self.seg_max[k * 2 + 1], self.seg_max[k * 2 + 2]);
        }
    }

    pub fn get_min(&self, a: usize, b: usize) -> i64 {
        self.get_min_range(a, b, 0, 0, NUM)
    }

    fn get_min_range(&self, a: usize, b: usize, k: usize, left: usize, right: usize) -> i64 {
        if b <= left || right <= a {
            INF
        } else if a <= left && right <= b {
            self.seg_min[k] + self.seg_add[k]
        } else {
            let x = self.get_min_range(a, b, k * 2 + 1, left, (left + right) / 2);
            let y = self.get_min_range(a, b, k * 2 + 2, (left + right) / 2, right);
            cmp::min(x, y) + self.seg_add[k]
        }
    }

    pub fn get_max(&self, a: usize, b: usize) -> i64 {
        self.get_max_range(a, b, 0, 0, NUM)
    }

    fn get_max_range(&self, a: usize, b: usize, k: usize, left: usize, right: usize) -> i64 {
        if b <= left || right <= a {
            -INF
        } else if a <= left && right <= b {
            self.seg_max[k] + self.seg_add[k]
        } else {
            let x = self.get_max_range(a, b, k * 2 + 1, left, (left + right) / 2);
            let y = self.get_max_range(a, b, k * 2 + 2, (left + right) / 2, right);
            cmp::max(x, y) + self.seg_add[k]
        }
    }
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
