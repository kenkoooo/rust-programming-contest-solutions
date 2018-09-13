use std::cmp;
use std::collections::VecDeque;

trait TopQueue<T> {
    fn top(&self) -> &T;
}

impl<T> TopQueue<T> for VecDeque<T> {
    fn top(&self) -> &T {
        self.iter().next().unwrap()
    }
}

fn calc_area(height: usize, width: usize) -> usize {
    (height + 1) * (width + 1)
}

fn calc_max_rectangle(hist: &Vec<usize>) -> usize {
    let n = hist.len();
    let mut ans = 0;
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..n {
        let mut reachable_min = i;
        while !stack.is_empty() && stack.top().1 > hist[i] {
            let (pos, height) = stack.pop_front().unwrap();
            reachable_min = pos;

            let area = calc_area(height, i - reachable_min);
            ans = cmp::max(ans, area);
        }

        if stack.is_empty() || stack.top().1 < hist[i] {
            stack.push_front((reachable_min, hist[i]));
        }
    }

    while !stack.is_empty() {
        let (pos, height) = stack.pop_front().unwrap();

        let area = calc_area(n - pos, height);
        ans = cmp::max(ans, area);
    }
    ans
}

fn main() {
    let mut sc = Scanner::new();
    let h = sc.read();
    let w: usize = sc.read();
    let a: Vec<Vec<usize>> = (0..h)
        .map(|_| {
            sc.read::<String>()
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect()
        }).collect();

    let w = w - 1;
    let h = h - 1;
    let mut map = vec![vec![false; w]; h];

    for i in 0..h {
        for j in 0..w {
            let s = a[i][j] + a[i][j + 1] + a[i + 1][j] + a[i + 1][j + 1];
            map[i][j] = s % 2 == 0;
        }
    }

    let mut hist = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if !map[i][j] {
                continue;
            }
            if i == 0 {
                hist[i][j] = 1;
            } else {
                hist[i][j] = hist[i - 1][j] + 1;
            }
        }
    }

    let mut ans = cmp::max(w + 1, h + 1);
    for i in 0..h {
        ans = cmp::max(ans, calc_max_rectangle(&hist[i]));
    }
    println!("{}", ans);
}

trait CppDeque<S> {
    fn top(&mut self) -> S;
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
