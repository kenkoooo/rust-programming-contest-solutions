fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let h: usize = sc.read();
    let w: usize = sc.read();
    let q: usize = sc.read();
    let s = (0..h)
        .map(|_| {
            sc.read::<String>()
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut vertical = vec![vec![0; w]; h];
    for i in 0..(h - 1) {
        for j in 0..w {
            vertical[i][j] = if s[i][j] == 1 && s[i + 1][j] == 1 {
                1
            } else {
                0
            };
        }
    }

    let mut horizontal = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..(w - 1) {
            horizontal[i][j] = if s[i][j] == 1 && s[i][j + 1] == 1 {
                1
            } else {
                0
            };
        }
    }

    let vertical = CumulativeSum::new(&vertical, 0);
    let horizontal = CumulativeSum::new(&horizontal, 0);
    let sum = CumulativeSum::new(&s, 0);

    for _ in 0..q {
        let x1 = sc.read::<usize>() - 1;
        let y1 = sc.read::<usize>() - 1;
        let x2 = sc.read::<usize>() - 1;
        let y2 = sc.read::<usize>() - 1;
        let v = sum.get_sum(x1, y1, x2, y2);
        let e1 = if x1 < x2 {
            vertical.get_sum(x1, y1, x2 - 1, y2)
        } else {
            0
        };
        let e2 = if y1 < y2 {
            horizontal.get_sum(x1, y1, x2, y2 - 1)
        } else {
            0
        };
        //        println!();
        //        for i in x1..(x2 + 1) {
        //            for j in y1..(y2 + 1) {
        //                print!("{}", s[i][j]);
        //            }
        //            println!();
        //        }
        //        println!("v={} ev={} eh={}", v, e1, e2);
        //        println!("{}", v - e1 - e2);

        sc.write(format!("{}\n", v - e1 - e2));
    }
}

pub struct CumulativeSum<T> {
    ny: usize,
    nx: usize,
    sum: Vec<Vec<T>>,
}

impl<T> CumulativeSum<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    pub fn new(a: &Vec<Vec<T>>, init: T) -> CumulativeSum<T> {
        assert!(a.len() > 0);
        let ny = a.len();
        let nx = a[0].len();
        let mut sum = vec![vec![init; nx + 1]; ny + 1];
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

    pub fn get_sum(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> T {
        assert!(y1 <= y2 && x1 <= x2);
        assert!(y2 <= self.ny - 1);
        assert!(x2 <= self.nx - 1);
        return self.sum[y2 + 1][x2 + 1] + self.sum[y1][x1]
            - self.sum[y1][x2 + 1]
            - self.sum[y2 + 1][x1];
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
