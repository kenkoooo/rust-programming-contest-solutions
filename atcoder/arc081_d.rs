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
    let mut rectangle = vec![vec![false; w - 1]; h - 1];
    for h in 0..(h - 1) {
        for w in 0..(w - 1) {
            let mut count = 0;
            for h in h..(h + 2) {
                for w in w..(w + 2) {
                    if s[h][w] == '#' {
                        count += 1;
                    }
                }
            }
            rectangle[h][w] = count % 2 == 0;
        }
    }

    let mut ans = max_rectangle::maximize(&rectangle);
    ans = cmp::max(ans, h);
    ans = cmp::max(ans, w);

    println!("{}", ans);
}

pub mod max_rectangle {
    use std::cmp;
    use std::collections::VecDeque;

    fn calc_area(h: usize, w: usize) -> usize {
        (h + 1) * (w + 1)
    }

    fn calc(hist: &Vec<usize>) -> usize {
        let n = hist.len();
        let mut ans = 0;
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        for i in 0..n {
            let mut reachable_min = i;
            while let Some((pos, height)) = q.pop_front() {
                if height <= hist[i] {
                    q.push_front((pos, height));
                    break;
                }
                reachable_min = pos;
                ans = cmp::max(
                    ans,
                    /* (i - reachable_min) * height */ calc_area(i - reachable_min, height),
                );
            }

            if q.is_empty() || q.iter().next().unwrap().1 < hist[i] {
                q.push_front((reachable_min, hist[i]));
            }
        }
        while let Some((pos, height)) = q.pop_front() {
            ans = cmp::max(
                ans,
                /* (n - pos) * height */ calc_area(n - pos, height),
            );
        }
        ans
    }

    pub fn maximize(map: &Vec<Vec<bool>>) -> usize {
        let h = map.len();
        let w = map[0].len();
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

        let mut ans = 0;
        for i in 0..h {
            ans = cmp::max(ans, calc(&hist[i]));
        }
        ans
    }
}
