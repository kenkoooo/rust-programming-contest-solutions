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

const INF: usize = 1e15 as usize;

fn solve1(xy: &Vec<(usize, usize)>) -> usize {
    let max = xy.iter().map(|&(x, y)| cmp::max(x, y)).max().unwrap();
    let min = xy.iter().map(|&(x, y)| cmp::min(x, y)).min().unwrap();
    let max_minimum = xy.iter().map(|&(x, y)| cmp::min(x, y)).max().unwrap();
    let min_maximum = xy.iter().map(|&(x, y)| cmp::max(x, y)).min().unwrap();
    (max - min_maximum) * (max_minimum - min)
}

fn solve2(xy: &Vec<(usize, usize)>) -> usize {
    let n = xy.len();
    let mut pairs = vec![];
    for &(x, y) in xy.iter() {
        pairs.push((cmp::min(x, y), cmp::max(x, y)));
    }
    pairs.sort();

    let (whole_minimum, _) = pairs[0];
    let whole_maximum = pairs.iter().map(|&(_, b)| b).max().unwrap();
    let (mut cur_max, _) = pairs[n - 1];
    let mut right_min = INF;

    let mut ans = (cur_max - pairs[0].0) * (whole_maximum - whole_minimum);
    for i in 0..(n - 1) {
        let (_, cur_right) = pairs[i];
        let (next_left, _) = pairs[i + 1];
        right_min = cmp::min(right_min, cur_right);
        let cur_min = cmp::min(right_min, next_left);
        cur_max = cmp::max(cur_max, cur_right);
        ans = cmp::min(ans, (whole_maximum - whole_minimum) * (cur_max - cur_min));
    }
    ans
}

fn main() {
    input!(n: usize, xy: [(usize, usize); n]);
    println!("{}", cmp::min(solve1(&xy), solve2(&xy)));
}
