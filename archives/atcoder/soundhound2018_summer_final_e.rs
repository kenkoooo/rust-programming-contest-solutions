const EXP: usize = 1e6 as usize;
const MOD: usize = 1e9 as usize + 7;

#[derive(Clone, Debug)]
struct Node {
    left: usize,
    right: usize,
    from: usize,
    to: usize,
    sum: usize,
}

struct SegTree {
    nodes: Vec<Node>,
    total: usize,
}

impl SegTree {
    fn new(n: usize, m: usize) -> Self {
        let mut size = 1;
        while size <= n {
            size *= 2;
        }
        let total = size * 2;
        let mut seg = vec![
            Node {
                left: 0,
                right: 0,
                from: 0,
                to: 0,
                sum: 0
            };
            total * m
        ];

        for m in 0..m {
            let offset = total * m;
            init_ptr(&mut seg, 0, offset, 0, size - 1);
        }
        SegTree {
            nodes: seg,
            total: total,
        }
    }

    fn update(&mut self, pos: usize, value: usize, i: usize) {
        if self.nodes[pos].from == self.nodes[pos].to && self.nodes[pos].from == i {
            self.nodes[pos].sum = value;
        } else if self.nodes[pos].from <= i && i <= self.nodes[pos].to {
            let left = self.nodes[pos].left;
            let right = self.nodes[pos].right;
            self.update(left, value, i);
            self.update(right, value, i);
            self.nodes[pos].sum = (self.nodes[left].sum + self.nodes[right].sum) % MOD;
        }
    }

    fn sum(&self, from: usize, to: usize, pos: usize) -> usize {
        if to < self.nodes[pos].from || self.nodes[pos].to < from {
            0
        } else if from <= self.nodes[pos].from && self.nodes[pos].to <= to {
            self.nodes[pos].sum
        } else {
            let left = self.sum(from, to, self.nodes[pos].left);
            let right = self.sum(from, to, self.nodes[pos].right);
            (left + right) % MOD
        }
    }

    fn swap(&mut self, pos1: usize, pos2: usize, from: usize, to: usize) {
        let ptr1 = self.nodes[pos1].left;
        let ptr2 = self.nodes[pos2].left;
        if from <= self.nodes[ptr1].from && self.nodes[ptr1].to <= to {
            self.nodes[pos1].left = ptr2;
            self.nodes[pos2].left = ptr1;
        } else if !(to < self.nodes[ptr1].from || self.nodes[ptr1].to < from) {
            self.swap(ptr1, ptr2, from, to);
        }
        let ptr1 = self.nodes[pos1].right;
        let ptr2 = self.nodes[pos2].right;
        if from <= self.nodes[ptr1].from && self.nodes[ptr1].to <= to {
            self.nodes[pos1].right = ptr2;
            self.nodes[pos2].right = ptr1;
        } else if !(to < self.nodes[ptr1].from || self.nodes[ptr1].to < from) {
            self.swap(ptr1, ptr2, from, to);
        }
        let mut update = |pos: usize| {
            let left = self.nodes[pos].left;
            let right = self.nodes[pos].right;
            self.nodes[pos].sum = (self.nodes[left].sum + self.nodes[right].sum) % MOD;
        };

        update(pos1);
        update(pos2);
    }
}

fn init_ptr(seg: &mut Vec<Node>, k: usize, offset: usize, from: usize, to: usize) {
    seg[k + offset].from = from;
    seg[k + offset].to = to;
    if from != to {
        let width = to + 1 - from;
        assert_eq!(width & 1, 0);
        seg[k + offset].left = 2 * k + 1 + offset;
        seg[k + offset].right = 2 * k + 2 + offset;
        init_ptr(seg, 2 * k + 1, offset, from, from + width / 2 - 1);
        init_ptr(seg, 2 * k + 2, offset, from + width / 2, to);
    }
}

fn solve(n: usize, s: &Vec<Vec<usize>>, q: &Vec<(usize, usize, usize, usize, usize)>) {
    let m = s.len();
    let mut mod_pow = vec![0; n + 1];
    mod_pow[0] = 1;
    mod_pow[1] = EXP;
    let mut mod_inv = vec![0; n + 1];
    mod_inv[0] = 1;
    mod_inv[1] = mod_inverse(EXP, MOD);
    for i in 2..n {
        mod_pow[i] = (mod_pow[i - 1] * mod_pow[1]) % MOD;
        mod_inv[i] = (mod_inv[i - 1] * mod_inv[1]) % MOD;
    }

    let mut seg = SegTree::new(n, m);
    for m in 0..m {
        for i in 0..n {
            let pos = m * seg.total;
            seg.update(pos, (s[m][i] * mod_pow[i]) % MOD, i);
        }
    }

    for &(t, x, y, from, to) in q.iter() {
        if t == 1 {
            let pos1 = x * seg.total;
            let pos2 = y * seg.total;
            seg.swap(pos1, pos2, from, to);
        } else {
            let pos = x * seg.total;
            let sum = seg.sum(from, to, pos);
            println!("{}", (sum * mod_inv[from]) % MOD);
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();
    let s: Vec<Vec<usize>> = (0..m)
        .map(|_| {
            sc.read::<String>()
                .chars()
                .map(|c| c as usize - 'a' as usize + 1)
                .collect()
        })
        .collect();
    let q = sc.read();
    let q = (0..q)
        .map(|_| {
            let t = sc.read();
            if t == 1 {
                (
                    t,
                    sc.usize_read() - 1,
                    sc.usize_read() - 1,
                    sc.usize_read() - 1,
                    sc.usize_read() - 1,
                )
            } else {
                (
                    t,
                    sc.usize_read() - 1,
                    sc.usize_read(),
                    sc.usize_read() - 1,
                    sc.usize_read() - 1,
                )
            }
        })
        .collect();
    solve(n, &s, &q);
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    assert!(a < b);
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(a: usize, modulo: usize) -> usize {
    let (g, x, _) = extended_gcd(a as i64, modulo as i64);
    assert!(g == 1);
    (x as usize) % modulo
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
