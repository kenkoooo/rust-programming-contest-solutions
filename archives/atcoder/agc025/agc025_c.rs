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

use std::collections::BTreeSet;

struct Data {
    positive: BTreeSet<(i64, i64, usize)>,
    negative: BTreeSet<(i64, i64, usize)>,
}

impl Data {
    fn new(lr: &Vec<(i64, i64)>) -> Self {
        let mut positive_set = BTreeSet::new();
        let mut negative_set = BTreeSet::new();
        let n = lr.len();
        for i in 0..n {
            let (l, r) = lr[i];
            positive_set.insert((l, r, i));
            negative_set.insert((r, l, i));
        }
        Data {
            positive: positive_set,
            negative: negative_set,
        }
    }

    fn pop_min(&mut self) -> (i64, i64) {
        assert_eq!(self.positive.len(), self.negative.len());
        assert!(!self.negative.is_empty());
        let &(r, l, i) = self.negative.iter().next().unwrap();
        self.negative.remove(&(r, l, i));
        self.positive.remove(&(l, r, i));
        (l, r)
    }
    fn pop_max(&mut self) -> (i64, i64) {
        assert_eq!(self.positive.len(), self.negative.len());
        assert!(!self.positive.is_empty());
        let &(l, r, i) = self.positive.iter().next_back().unwrap();
        self.negative.remove(&(r, l, i));
        self.positive.remove(&(l, r, i));
        (l, r)
    }

    fn len(&self) -> usize {
        assert_eq!(self.positive.len(), self.negative.len());
        self.negative.len()
    }
}

fn solve(mut data: Data) -> i64 {
    let mut cur = 0;
    let mut ans = 0;
    let n = data.len();
    for i in 0..n {
        let (l, r) = if i % 2 == 0 {
            data.pop_max()
        } else {
            data.pop_min()
        };
        assert!(l < r);
        let (move_dist, next) = if cur < l {
            (l - cur, l)
        } else if r < cur {
            (cur - r, r)
        } else {
            (0, cur)
        };
        ans += move_dist;
        cur = next;
    }
    ans + (cur - 0).abs()
}

use std::cmp;

fn main() {
    input!(n: usize, lr: [(i64, i64); n]);
    let data = Data::new(&lr);
    let ans1 = solve(data);

    let neg_lr = lr.iter().map(|&(l, r)| (-r, -l)).collect::<Vec<_>>();
    let data = Data::new(&neg_lr);
    let ans2 = solve(data);
    println!("{}", cmp::max(ans1, ans2));
}
