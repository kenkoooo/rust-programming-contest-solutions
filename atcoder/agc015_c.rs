use std::cmp;
fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let m = sc.usize_read();
    let q = sc.usize_read();
    let s: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            sc.read::<String>()
                .chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut vertical = vec![vec![0; m]; n - 1];
    for i in 0..(n - 1) {
        for j in 0..m {
            if s[i][j] == 1 && s[i + 1][j] == 1 {
                vertical[i][j] = 1;
            }
        }
    }

    let mut horizontal = vec![vec![0; m - 1]; n];
    for i in 0..n {
        for j in 0..(m - 1) {
            if s[i][j] == 1 && s[i][j + 1] == 1 {
                horizontal[i][j] = 1;
            }
        }
    }

    let sum_s = CumulativeSum::new(&s);
    let sum_v = CumulativeSum::new(&vertical);
    let sum_h = CumulativeSum::new(&horizontal);

    for _ in 0..q {
        let x1 = sc.usize_read() - 1;
        let y1 = sc.usize_read() - 1;
        let x2 = sc.usize_read() - 1;
        let y2 = sc.usize_read() - 1;

        let v = sum_s.get_sum(x1, y1, x2, y2);
        let edge_h = if y2 >= 1 {
            sum_h.get_sum(x1, y1, x2, y2 - 1)
        } else {
            0
        };
        let edge_v = if x2 >= 1 {
            sum_v.get_sum(x1, y1, x2 - 1, y2)
        } else {
            0
        };

        println!("{}", v - edge_v - edge_h);
    }
}

pub struct CumulativeSum {
    ny: usize,
    nx: usize,
    sum: Vec<Vec<usize>>,
}

impl CumulativeSum {
    pub fn new(a: &Vec<Vec<usize>>) -> CumulativeSum {
        let ny = a.len();
        let nx = if ny == 0 { 0 } else { a[0].len() };
        let mut sum = vec![vec![0; nx + 1]; ny + 1];
        for i in 0..ny {
            for j in 0..nx {
                sum[i + 1][j + 1] = a[i][j] + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
            }
        }
        CumulativeSum {
            ny: ny,
            nx: nx,
            sum: sum,
        }
    }

    pub fn get_sum(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> usize {
        if y1 > y2 || x1 > x2 {
            return 0;
        }
        let y2 = cmp::min(y2, self.ny - 1);
        let x2 = cmp::min(x2, self.nx - 1);
        return self.sum[y2 + 1][x2 + 1] + self.sum[y1][x1] - self.sum[y1][x2 + 1]
            - self.sum[y2 + 1][x1];
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
