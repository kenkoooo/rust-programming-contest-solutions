/// Thank you tanakh!!!
///  https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

fn main() {
    input!(
        n: usize,
        q: usize,
        x: [usize; n],
        queries: [(usize, usize); q]
    );
    let x: Vec<usize> = x.iter().map(|&x| x + 1).collect();
    let queries: Vec<(usize, usize)> = queries.iter().map(|&(c, d)| (c + 1, d)).collect();
    //  -d<=x-c<=d
    // c-d<=x<=c+d

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + x[i];
    }

    for &(c, d) in queries.iter() {
        let low = if c <= d { 1 } else { c - d };
        let high = c + d + 1;
        let lower = x.lower_bound(low);
        let upper = x.lower_bound(high);
        let center_lower = x.lower_bound(c);
        let center_upper = x.lower_bound(c + 1);

        let lower_half = sum[center_lower] - sum[lower];
        let upper_half = sum[upper] - sum[center_upper];

        let lower_count = center_lower - lower;
        let upper_count = upper - center_upper;
        let outside = n - (upper - lower);

        let ans = outside * d + (c * lower_count - lower_half) + (upper_half - c * upper_count);
        println!("{}", ans);
    }
}

trait VecBound<T> {
    fn lower_bound(&self, key: T) -> usize;
}

impl VecBound<usize> for Vec<usize> {
    fn lower_bound(&self, key: usize) -> usize {
        assert!(key > 0);
        self.binary_search_by_key(&(key * 2 - 1), |&x| x * 2)
            .err()
            .unwrap()
    }
}
