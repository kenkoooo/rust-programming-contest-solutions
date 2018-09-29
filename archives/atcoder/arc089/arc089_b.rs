use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let k = sc.read::<usize>();
    let xyc = (0..n).map(|_| {
        let x = sc.read::<usize>() % (2 * k);
        let y = sc.read::<usize>() % (2 * k);
        let c = sc.read::<String>() == "B";
        (x, y, c)
    }).collect::<Vec<_>>();

    let (black, white) = {
        let mut b = vec![vec![0; k]; k];
        let mut w = vec![vec![0; k]; k];
        for t in &xyc {
            let (x, y, mut c) = *t;
            if (x < k && y < k) || (x >= k && y >= k) {
                c = !c;
            }

            if c {
                b[x % k][y % k] += 1;
            } else {
                w[x % k][y % k] += 1;
            }
        }
        let black = CumulativeSum::new(&b);
        let white = CumulativeSum::new(&w);
        (black, white)
    };

    let mut ans = 0;
    for i in 0..k {
        for j in 0..k {
            let b1 = black.get_sum(0, 0, i, j);
            let b2 = black.get_sum(i + 1, j + 1, k - 1, k - 1);

            let w1 = white.get_sum(0, 0, i, j);
            let w2 = white.get_sum(i + 1, j + 1, k - 1, k - 1);

            let bc = b1 + b2 + white.get_sum(0, 0, k - 1, k - 1) - w1 - w2;
            let wc = w1 + w2 + black.get_sum(0, 0, k - 1, k - 1) - b1 - b2;
            ans = cmp::max(ans, bc);
            ans = cmp::max(ans, wc);
        }
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
        CumulativeSum { ny: ny, nx: nx, sum: sum }
    }

    fn get_sum(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> usize {
        if y1 > y2 || x1 > x2 {
            return 0;
        }
        let y2 = cmp::min(y2, self.ny - 1);
        let x2 = cmp::min(x2, self.nx - 1);
        return self.sum[y2 + 1][x2 + 1] + self.sum[y1][x1] - self.sum[y1][x2 + 1] - self.sum[y2 + 1][x1];
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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
        }
        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

