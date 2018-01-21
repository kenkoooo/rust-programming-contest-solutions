use std::io;
use std::cmp;

fn main() {
    let (n, k) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };

    let xyc = (0..n).map(|_| {
        let v = read_values::<String>();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        let c = (v[2] == "B");
        if !c {
            assert_eq!(v[2], "W");
        }
        (x % (2 * k), y % (2 * k), c)
    }).collect::<Vec<_>>();
    println!("{}", solve(&xyc, k));
}

fn solve(xyc: &Vec<(usize, usize, bool)>, k: usize) -> usize {
    let n = xyc.len();
    let mut b = vec![vec![0; k]; k];
    let mut w = vec![vec![0; k]; k];

    for i in 0..n {
        let (x, y, c) = xyc[i];

        if x >= k && y >= k {
            if c {
                b[x % k][y % k] += 1;
            } else {
                w[x % k][y % k] += 1;
            }
        } else if x >= k {
            if c {
                w[x % k][y] += 1;
            } else {
                b[x % k][y] += 1;
            }
        } else if y >= k {
            if c {
                w[x][y % k] += 1;
            } else {
                b[x][y % k] += 1;
            }
        } else {
            if c {
                b[x][y] += 1;
            } else {
                w[x][y] += 1;
            }
        }
    }

    let b_sum = CumulativeSum::new(&b);
    let w_sum = CumulativeSum::new(&w);

    let w_all = w_sum.get_sum(0, 0, k - 1, k - 1);
    let b_all = b_sum.get_sum(0, 0, k - 1, k - 1);
    assert_eq!(w_all + b_all, n);

    let mut ans = cmp::max(w_all, b_all);
    for i in 0..k {
        for j in 0..k {
            let b_up = b_sum.get_sum(i, j, k - 1, k - 1);
            let b_low = if i == 0 || j == 0 {
                0
            } else {
                b_sum.get_sum(0, 0, i - 1, j - 1)
            };

            let w_up = w_sum.get_sum(i, j, k - 1, k - 1);
            let w_low = if i == 0 || j == 0 {
                0
            } else {
                w_sum.get_sum(0, 0, i - 1, j - 1)
            };

            let ok = b_up + b_low + w_all - w_up - w_low;
            assert!(w_all >= w_up + w_low);
            ans = cmp::max(ans, ok);

            let ok = w_up + w_low + (n - w_all) - b_up - b_low;
            ans = cmp::max(ans, ok);
        }
    }
    assert!(ans <= n);
    ans
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

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}