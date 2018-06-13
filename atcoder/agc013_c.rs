fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let l = sc.usize_read();
    let t = sc.usize_read();
    let xw: Vec<(usize, bool)> = (0..n).map(|_| (sc.read(), sc.usize_read() == 1)).collect();

    let (first_pos, first_clock) = xw[0];
    let mut conflict_first = 0;
    for i in 1..n {
        let (x, w) = xw[i];
        if w == first_clock {
            continue;
        }
        let interval = if first_clock {
            x - first_pos
        } else {
            l + first_pos - x
        };

        if interval > t * 2 {
            continue;
        }
        conflict_first += (2 * t - interval) / l + 1;
        if (2 * t - interval) % l == 0 && first_clock {
            conflict_first -= 1;
        }
    }

    let mut moved = vec![];
    for &(x, w) in &xw {
        if w {
            let x = (x + t) % l;
            moved.push(x);
        } else {
            let x = (l + x - t % l) % l;
            moved.push(x);
        }
    }
    moved.sort();
    let moved_first = if first_clock {
        (first_pos + t) % l
    } else {
        (l + first_pos - t % l) % l
    };

    let mut moved_first_i = 0;
    for i in 0..n {
        if moved_first == moved[i] {
            moved_first_i = i;
            break;
        }
    }
    let cur_id = if first_clock {
        conflict_first % n
    } else {
        (n - conflict_first % n) % n
    };

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[(cur_id + i) % n] = moved[(moved_first_i + i) % n];
    }

    for &ans in &ans {
        println!("{}", ans);
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
