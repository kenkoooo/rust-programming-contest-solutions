use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut cur_h: f64 = sc.read();
    let q = sc.read();
    let mut t_h: Vec<(usize, usize)> = (0..q).map(|_| (sc.read(), sc.read())).collect();
    t_h.sort();

    let mut check = false;
    for &(_, h) in t_h.iter() {
        if h == 0 {
            check = true;
        }
    }
    if !check {
        println!("-1");
        return;
    }

    let mut heap = BinaryHeap::new();

    let mut cur_t = 0.0;
    let mut decrease_per_h = 0.0;
    for &(t, h) in t_h.iter() {
        let t = t as f64;
        let h = h as f64;

        while let Some(MinNonNan(h)) = heap.pop() {
            if decrease_per_h == 0.0 {
                heap.push(MinNonNan(h));
                break;
            }
            assert!(h <= cur_h);
            let next_pop_out_time = (cur_h - h) / decrease_per_h + cur_t;
            if t >= next_pop_out_time {
                decrease_per_h -= 1.0;
                cur_t = next_pop_out_time;
                cur_h = h;

                // eprintln!("cur_h={} cur_t={} leak={}", cur_h, cur_t, decrease_per_h);
            } else {
                heap.push(MinNonNan(h));
                break;
            }
        }

        if cur_h == 0.0 {
            println!("{}", cur_t);
            return;
        }

        let decrease = (t - cur_t) * decrease_per_h;
        if decrease > cur_h {
            break;
        } else {
            cur_h -= decrease;
        }

        cur_t = t;

        if h < cur_h {
            heap.push(MinNonNan(h));
            decrease_per_h += 1.0;
        }

        // eprintln!("cur_h={} cur_t={} leak={}", cur_h, cur_t, decrease_per_h);
    }

    while let Some(MinNonNan(h)) = heap.pop() {
        if decrease_per_h == 0.0 {
            heap.push(MinNonNan(h));
            break;
        }
        assert!(h <= cur_h);
        let next_pop_out_time = (cur_h - h) / decrease_per_h + cur_t;
        decrease_per_h -= 1.0;
        cur_t = next_pop_out_time;
        cur_h = h;

        // eprintln!("cur_h={} cur_t={} leak={}", cur_h, cur_t, decrease_per_h);
    }

    println!("{}", cur_t);
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
