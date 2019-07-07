fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let q: usize = sc.read();
    let s: Vec<Vec<_>> = (0..n)
        .map(|_| {
            sc.chars()
                .into_iter()
                .map(|c| c as u32 - '0' as u32)
                .collect()
        })
        .collect();

    let sum = CumulativeSum::new(&s, 0);
    let h_sum = match n {
        1 => None,
        _ => {
            let mut h_edges: Vec<Vec<u32>> = vec![vec![0; m]; n - 1];
            for i in 1..n {
                for j in 0..m {
                    h_edges[i - 1][j] = if s[i][j] == 1 && s[i - 1][j] == 1 {
                        1
                    } else {
                        0
                    };
                }
            }
            Some(CumulativeSum::new(&h_edges, 0))
        }
    };

    let v_sum = match m {
        1 => None,
        _ => {
            let mut v_edges: Vec<Vec<u32>> = vec![vec![0; m - 1]; n];
            for i in 0..n {
                for j in 1..m {
                    v_edges[i][j - 1] = if s[i][j - 1] == 1 && s[i][j] == 1 {
                        1
                    } else {
                        0
                    };
                }
            }
            Some(CumulativeSum::new(&v_edges, 0))
        }
    };

    for _ in 0..q {
        let x1 = sc.read::<usize>() - 1;
        let y1 = sc.read::<usize>() - 1;
        let x2 = sc.read::<usize>() - 1;
        let y2 = sc.read::<usize>() - 1;

        let mut v = sum.get_sum(x1, y1, x2, y2);
        match v_sum {
            Some(ref v_sum) if y1 < y2 => {
                v -= v_sum.get_sum(x1, y1, x2, y2 - 1);
            }
            _ => {}
        }
        match h_sum {
            Some(ref h_sum) if x1 < x2 => {
                v -= h_sum.get_sum(x1, y1, x2 - 1, y2);
            }
            _ => {}
        }
        println!("{}", v);
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
pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
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
