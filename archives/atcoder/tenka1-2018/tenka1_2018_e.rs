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
    input!(h: usize, w: usize, s: [chars; h]);

    let s: Vec<Vec<usize>> = s
        .iter()
        .map(|t| t.iter().map(|&c| if c == '#' { 1 } else { 0 }).collect())
        .collect();

    let ans1 = solve(h, w, &s);
    let s: Vec<Vec<usize>> = s
        .iter()
        .map(|t| {
            let mut t = t.clone();
            t.reverse();
            t
        }).collect();
    let ans2 = solve(h, w, &s);

    let mut ans3 = count_duplicated(h, w, &s);
    println!("{}", ans1 + ans2 - ans3);
}

fn solve(h: usize, w: usize, s: &Vec<Vec<usize>>) -> usize {
    let x = cmp::max(h, w);
    let mut map = vec![vec![0; x * 3]; x * 3];
    for i in 0..h {
        for j in 0..w {
            map[i + x][j + x] = s[i][j];
        }
    }
    let z = x * 3;
    let mut sum = vec![vec![0; z + 1]; z + 1];
    for si in 0..z {
        for i in si..z {
            let j = i - si;
            sum[i + 1][j + 1] = sum[i][j] + map[i][j];
        }
    }

    for sj in 1..z {
        for j in sj..z {
            let i = j - sj;
            sum[i + 1][j + 1] = sum[i][j] + map[i][j];
        }
    }

    let mut ans = 0;
    for si in 0..h {
        for i1 in si..h {
            let j1 = i1 - si;
            if j1 >= w {
                continue;
            }
            for i2 in (i1 + 1)..h {
                let j2 = i2 - si;
                if j2 >= w {
                    continue;
                }
                if s[i1][j1] != 1 || s[i2][j2] != 1 {
                    continue;
                }
                let d = i2 - i1;
                ans += sum[x + i2 + d + 1][x + j1 + 1] - sum[x + i2][x + j1 - d];
                ans += sum[x + i1 + 1][x + j2 + d + 1] - sum[x + i1 - d][x + j2];
            }
        }
    }

    for sj in 1..w {
        for j1 in sj..w {
            let i1 = j1 - sj;
            if i1 >= h {
                continue;
            }
            for j2 in (j1 + 1)..w {
                let i2 = j2 - sj;
                if i2 >= h {
                    continue;
                }
                if s[i1][j1] != 1 || s[i2][j2] != 1 {
                    continue;
                }
                let d = i2 - i1;
                ans += sum[x + i2 + d + 1][x + j1 + 1] - sum[x + i2][x + j1 - d];
                ans += sum[x + i1 + 1][x + j2 + d + 1] - sum[x + i1 - d][x + j2];
            }
        }
    }
    ans
}

fn count_duplicated(h: usize, w: usize, s: &Vec<Vec<usize>>) -> usize {
    let x = cmp::max(h, w);

    let mut ans = 0;
    for si in 0..h {
        for i1 in si..h {
            let j1 = i1 - si;
            if j1 >= w {
                continue;
            }
            for i2 in (i1 + 1)..h {
                let j2 = i2 - si;
                if j2 >= w {
                    continue;
                }
                if s[i1][j1] != 1 || s[i2][j2] != 1 {
                    continue;
                }
                let d = i2 - i1;
                if i1 >= d {
                    ans += s[i1 - d][j2];
                }
                if j2 + d < w {
                    ans += s[i1][j2 + d];
                }
                if j1 >= d {
                    ans += s[i2][j1 - d];
                }
                if i2 + d < h {
                    ans += s[i2 + d][j1];
                }
            }
        }
    }

    for sj in 1..w {
        for j1 in sj..w {
            let i1 = j1 - sj;
            if i1 >= h {
                continue;
            }
            for j2 in (j1 + 1)..w {
                let i2 = j2 - sj;
                if i2 >= h {
                    continue;
                }
                if s[i1][j1] != 1 || s[i2][j2] != 1 {
                    continue;
                }
                let d = i2 - i1;
                if i1 >= d {
                    ans += s[i1 - d][j2];
                }
                if j2 + d < w {
                    ans += s[i1][j2 + d];
                }
                if j1 >= d {
                    ans += s[i2][j1 - d];
                }
                if i2 + d < h {
                    ans += s[i2 + d][j1];
                }
            }
        }
    }
    ans
}
