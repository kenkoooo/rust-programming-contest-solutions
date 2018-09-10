use std::cmp;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Seg {
    head: usize,
    tail: usize,
}

impl Ord for Seg {
    fn cmp(&self, other: &Seg) -> cmp::Ordering {
        other.tail.cmp(&self.tail)
    }
}

impl PartialOrd for Seg {
    fn partial_cmp(&self, other: &Seg) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();
    let a: Vec<usize> = sc.read_vec(n);

    let mut tails: Vec<Vec<usize>> = vec![vec![]; m];
    let mut total_distance = 0;
    let mut current_decrease = 0;
    let mut decreased_segments = BinaryHeap::new();
    for i in 1..n {
        let from = a[i - 1] - 1;
        let to = a[i] - 1;
        let distance = (to + m - from) % m;
        total_distance += distance;
        if from > to {
            let warped = m - from - 1;
            current_decrease += warped;
            decreased_segments.push(Seg {
                head: from,
                tail: to,
            });
            tails[from].push(to + m);
        } else {
            tails[from].push(to);
        }
    }

    let mut ans = total_distance - current_decrease;
    for x in 1..m {
        while let Some(Seg { head, tail }) = decreased_segments.pop() {
            if tail < x {
                assert!(tail == x - 1);
                current_decrease -= (tail + m - head) % m - 1;
            } else {
                decreased_segments.push(Seg {
                    head: head,
                    tail: tail,
                });
                break;
            }
        }
        current_decrease += decreased_segments.len();
        ans = cmp::min(ans, total_distance - current_decrease);

        for &tail in tails[x - 1].iter() {
            decreased_segments.push(Seg {
                head: x - 1,
                tail: tail,
            });
        }
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
