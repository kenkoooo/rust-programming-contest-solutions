use std::collections::{BTreeSet, VecDeque};

fn main() {
    let mut sc = Scanner::new();
    let r = sc.usize_read();
    let c = sc.usize_read();
    let n = sc.usize_read();

    let on_line = |x: usize, y: usize| x == 0 || x == r || y == 0 || y == c;

    let mut upper_x = vec![];
    let mut upper_y = vec![];
    let mut lower_x = vec![];
    let mut lower_y = vec![];

    for i in 0..n {
        let x1 = sc.usize_read();
        let y1 = sc.usize_read();
        let x2 = sc.usize_read();
        let y2 = sc.usize_read();
        if on_line(x1, y1) && on_line(x2, y2) {
            let mut push = |i: usize, x: usize, y: usize| {
                if x == 0 {
                    lower_x.push((y, i));
                } else if x == r {
                    upper_x.push((y, i));
                } else if y == 0 {
                    lower_y.push((x, i));
                } else if y == c {
                    upper_y.push((x, i));
                } else {
                    panic!();
                };
            };

            push(i, x1, y1);
            push(i, x2, y2);
        }
    }

    upper_x.sort();
    lower_x.sort();
    upper_y.sort();
    lower_y.sort();

    let mut line = vec![];
    for &(_, i) in &lower_x {
        line.push(i);
    }
    for &(_, i) in &upper_y {
        line.push(i);
    }
    for &(_, i) in upper_x.iter().rev() {
        line.push(i);
    }
    for &(_, i) in lower_y.iter().rev() {
        line.push(i);
    }

    let mut stack = VecDeque::new();
    let mut set = BTreeSet::new();
    for &i in &line {
        if stack.is_empty() || head(&mut stack) != i {
            if set.contains(&i) {
                println!("NO");
                return;
            }

            stack.push_front(i);
            set.insert(i);
        } else {
            stack.pop_front();
        }
    }

    println!("YES");
}

fn head<T: Clone>(stack: &mut VecDeque<T>) -> T {
    let h = stack.pop_front().unwrap();
    stack.push_front(h.clone());
    h
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
