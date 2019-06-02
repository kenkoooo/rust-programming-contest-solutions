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

use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MinUsize(i64);

impl Ord for MinUsize {
    fn cmp(&self, other: &MinUsize) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for MinUsize {
    fn partial_cmp(&self, other: &MinUsize) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

use std::cmp;

fn main() {
    input!(n: usize, a: [i64; 3 * n]);

    let mut bigger = vec![0; n + 1];
    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        heap.push(MinUsize(a[i]));
    }
    for i in 0..(n + 1) {
        bigger[i] = sum;
        sum += a[i + n];
        heap.push(MinUsize(a[i + n]));
        let removed = heap.pop().unwrap().0;
        sum -= removed;
        assert_eq!(heap.len(), n);
    }

    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    let mut smaller = vec![0; n + 1];
    for i in 0..n {
        sum += a[3 * n - 1 - i];
        heap.push(a[3 * n - 1 - i]);
    }

    for i in 0..(n + 1) {
        smaller[i] = sum;
        let a = a[2 * n - 1 - i];
        sum += a;
        heap.push(a);
        let removed = heap.pop().unwrap();
        sum -= removed;
        assert_eq!(heap.len(), n);
    }

    let mut ans: i64 = std::i64::MIN;
    for i in 0..(n + 1) {
        ans = cmp::max(ans, bigger[n - i] - smaller[i]);
    }
    println!("{}", ans);
}
