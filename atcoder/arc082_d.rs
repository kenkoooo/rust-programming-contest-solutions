use std::cmp;

#[derive(Debug)]
enum Query {
    Rotate(i64),
    Answer(i64, i64),
}

fn main() {
    let mut sc = Scanner::new();
    let x: i64 = sc.read();
    let k = sc.usize_read();

    let mut queries = vec![];
    for _ in 0..k {
        let r = sc.read();
        queries.push(Query::Rotate(r));
    }

    let q = sc.usize_read();
    for _ in 0..q {
        let t = sc.read();
        let a = sc.read();
        queries.push(Query::Answer(t, a));
    }
    queries.sort_by_key(|value| match value {
        &Query::Rotate(time) => time,
        &Query::Answer(time, _) => time,
    });

    let mut high = x;
    let mut low = 0;
    let mut super_high = x;
    let mut super_low = 0;
    let mut decreasing = true;
    let mut prev = 0;
    for query in &queries {
        match query {
            &Query::Rotate(time) => {
                let da = if decreasing {
                    -(time - prev)
                } else {
                    (time - prev)
                };
                high = cmp::min(x, cmp::max(0, high + da));
                low = cmp::min(x, cmp::max(0, low + da));
                super_high = super_high + da;
                super_low = super_low + da;

                decreasing = !decreasing;
                prev = time;
            }
            &Query::Answer(time, amount) => {
                let da = if decreasing {
                    -(time - prev)
                } else {
                    (time - prev)
                };

                let super_cur = super_low + da + amount;
                let high = cmp::min(x, cmp::max(0, high + da));
                let low = cmp::min(x, cmp::max(0, low + da));
                println!("{}", cmp::min(high, cmp::max(low, super_cur)));
            }
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
