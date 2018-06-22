use std::cmp;
fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();
    let q = sc.read();
    let map: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            sc.read::<String>()
                .chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut horizontal = vec![vec![0; m - 1]; n];
    let mut vertical = vec![vec![0; m]; n - 1];
    for i in 0..n {
        for j in 0..m {
            if j < m - 1 && map[i][j] == 1 && map[i][j + 1] == 1 {
                horizontal[i][j] = 1;
            }
            if i < n - 1 && map[i][j] == 1 && map[i + 1][j] == 1 {
                vertical[i][j] = 1;
            }
        }
    }

    let map = CumulativeSum::new(&map);
    let horizontal = if m > 1 {
        Some(CumulativeSum::new(&horizontal))
    } else {
        None
    };
    let vertical = if n > 1 {
        Some(CumulativeSum::new(&vertical))
    } else {
        None
    };

    for _ in 0..q {
        let x1 = sc.usize_read() - 1;
        let y1 = sc.usize_read() - 1;
        let x2 = sc.usize_read() - 1;
        let y2 = sc.usize_read() - 1;

        let cells = map.get_sum(x1, y1, x2, y2);
        let h = if y2 > 0 && horizontal.is_some() {
            horizontal.as_ref().unwrap().get_sum(x1, y1, x2, y2 - 1)
        } else {
            0
        };

        let v = if x2 > 0 && vertical.is_some() {
            vertical.as_ref().unwrap().get_sum(x1, y1, x2 - 1, y2)
        } else {
            0
        };
        println!("{}", cells - h - v);
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
        let nx = a[0].len();
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
