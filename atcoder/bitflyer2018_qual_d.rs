use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.read();
    let w: usize = sc.read();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let is_black: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            sc.read::<String>()
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let sum = CumulativeSum::new(&is_black);
    let mut ans = 0;
    {
        let h = cmp::min(2 * n, h);
        let w = cmp::min(2 * m, w);
        for i in 0..h {
            for j in 0..w {
                let from_i = if i < n {
                    let dh = cmp::min(i, h - n);
                    i - dh
                } else {
                    n - 1 - (h - i - 1)
                };

                let to_i = if i < n { i } else { n - 1 };

                let from_j = if j < m {
                    let dw = cmp::min(j, w - m);
                    j - dw
                } else {
                    m - 1 - (w - j - 1)
                };

                let to_j = if j < m { j } else { m - 1 };

                let s = sum.get_sum(from_i, from_j, to_i, to_j);
                let c = if s > 0 { 1 } else { 0 };
                ans += c;
            }
        }
    }

    if w > m * 2 {
        let w = w - m * 2;
        let h = cmp::min(2 * n, h);
        for i in 0..h {
            let from_i = if i < n {
                let dh = cmp::min(i, h - n);
                i - dh
            } else {
                n - 1 - (h - i - 1)
            };

            let to_i = if i < n { i } else { n - 1 };
            let c = if sum.get_sum(from_i, 0, to_i, m - 1) > 0 {
                1
            } else {
                0
            };
            ans += c * w;
        }
    }

    if h > n * 2 {
        let h = h - n * 2;
        let w = cmp::min(2 * m, w);
        for j in 0..w {
            let from_j = if j < m {
                let dw = cmp::min(j, w - m);
                j - dw
            } else {
                m - 1 - (w - j - 1)
            };

            let to_j = if j < m { j } else { m - 1 };
            let c = if sum.get_sum(0, from_j, n - 1, to_j) > 0 {
                1
            } else {
                0
            };
            ans += c * h;
        }
    }

    if h > n * 2 && w > m * 2 {
        let c = if sum.get_sum(0, 0, n - 1, m - 1) > 0 {
            1
        } else {
            0
        };
        ans += c * (h - 2 * n) * (w - 2 * m);
    }

    println!("{}", ans);
}

struct CumulativeSum {
    ny: usize,
    nx: usize,
    sum: Vec<Vec<usize>>,
}

impl CumulativeSum {
    fn new(a: &Vec<Vec<usize>>) -> CumulativeSum {
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

    fn get_sum(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> usize {
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
