/// Thank you tanakh!!!
/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::cmp;

fn main() {
    input!(n: usize, k: usize, xyc: [(usize, usize, String); n]);

    let mut black = vec![vec![0; k * 2]; k * 2];
    let mut white = vec![vec![0; k * 2]; k * 2];
    for &(x, y, ref c) in xyc.iter() {
        let x = x % (k * 2);
        let y = y % (k * 2);
        if c == "B" {
            black[x][y] += 1;
        } else {
            white[x][y] += 1;
        }
    }

    let l = k * 2;

    let black = CumulativeSum::new(&black);
    let white = CumulativeSum::new(&white);

    let all_black = black.get_sum(0, 0, l - 1, l - 1);
    let all_white = white.get_sum(0, 0, l - 1, l - 1);
    assert_eq!(all_black + all_white, n);

    let mut ans = 0;

    for i in 0..k {
        for j in 0..k {
            let x = i + k;
            let y = j + k;

            //e b
            // a
            //c d

            let mut included_black = 0;
            let mut excluded_white = 0;
            // a
            included_black += black.get_sum(i, j, x - 1, y - 1);
            excluded_white += white.get_sum(i, j, x - 1, y - 1);

            // b
            included_black += black.get_sum(x, y, l - 1, l - 1);
            excluded_white += white.get_sum(x, y, l - 1, l - 1);

            // c
            if i > 0 && j > 0 {
                included_black += black.get_sum(0, 0, i - 1, j - 1);
                excluded_white += white.get_sum(0, 0, i - 1, j - 1);
            }

            // d
            if j > 0 {
                included_black += black.get_sum(x, 0, l - 1, j - 1);
                excluded_white += white.get_sum(x, 0, l - 1, j - 1);
            }

            // e
            if i > 0 {
                included_black += black.get_sum(0, y, i - 1, l - 1);
                excluded_white += white.get_sum(0, y, i - 1, l - 1);
            }

            let ans1 = included_black + all_white - excluded_white;
            let ans2 = excluded_white + all_black - included_black;

            ans = cmp::max(ans, cmp::max(ans1, ans2));
        }
    }
    println!("{}", ans);
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
        return self.sum[y2 + 1][x2 + 1] + self.sum[y1][x1]
            - self.sum[y1][x2 + 1]
            - self.sum[y2 + 1][x1];
    }
}
