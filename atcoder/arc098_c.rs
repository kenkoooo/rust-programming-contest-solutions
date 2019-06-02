use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let q: usize = sc.read();
    let a: Vec<u64> = sc.read_vec(n);

    if n == 1 {
        println!("0");
        return;
    }

    let mut ans = std::u64::MAX;
    for &y in &a {
        let mut unremovable = vec![];
        for i in 0..n {
            if a[i] < y {
                unremovable.push(i);
            }
        }
        unremovable.push(n);

        let remove = |from: usize, to: usize| -> Vec<u64> {
            if from >= to || to - from + 1 < k {
                return Vec::new();
            }
            let mut x = vec![];
            for i in from..(to + 1) {
                x.push(a[i]);
            }
            x.sort();
            let count = x.len() - k + 1;
            (0..count).map(|i| x[i]).collect()
        };

        let mut removed = vec![];
        if unremovable[0] > 0 {
            removed.append(&mut remove(0, unremovable[0] - 1));
        }
        for i in 0..(unremovable.len() - 1) {
            let from = unremovable[i] + 1;
            let to = unremovable[i + 1] - 1;
            removed.append(&mut remove(from, to));
        }
        removed.sort();

        if removed.len() < q {
            continue;
        }
        let x = removed[q - 1];
        ans = cmp::min(ans, x - y);
    }

    println!("{}", ans);
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
