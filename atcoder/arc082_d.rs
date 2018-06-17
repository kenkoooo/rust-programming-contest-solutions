use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let x: i64 = sc.read();
    let k = sc.usize_read();
    let mut queries = vec![];
    for _ in 0..k {
        queries.push((sc.read::<i64>(), true, 0));
    }
    let q = sc.usize_read();
    for _ in 0..q {
        queries.push((sc.read::<i64>(), false, sc.read::<i64>()));
    }

    queries.sort();
    let mut super_low = 0;
    let mut super_high = x;
    let mut low = 0;
    let mut high = x;
    let mut decreasing = true;
    let mut last_turn = 0;
    for &(time, is_turn, a) in &queries {
        let dt = if decreasing {
            -(time - last_turn)
        } else {
            time - last_turn
        };
        if is_turn {
            high = cmp::max(0, cmp::min(x, high + dt));
            low = cmp::max(0, cmp::min(x, low + dt));
            super_high += dt;
            super_low += dt;
            decreasing = !decreasing;
            last_turn = time;
        } else {
            let cur_high = cmp::max(0, cmp::min(x, high + dt));
            let cur_low = cmp::max(0, cmp::min(x, low + dt));
            let cur_super_high = super_high + dt;
            let cur_super_low = super_low + dt;

            assert_eq!(cur_super_high - (x - a), cur_super_low + a);
            println!(
                "{}",
                cmp::min(cur_high, cmp::max(cur_low, cur_super_low + a))
            );
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
