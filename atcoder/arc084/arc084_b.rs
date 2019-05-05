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
use std::collections::BinaryHeap;

const INF: usize = 1e9 as usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    v: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input!(k: usize);

    let mut dist = vec![INF; k];
    let mut heap = BinaryHeap::new();
    for i in 1..10 {
        let remain = i % k;
        if dist[remain] > i {
            dist[remain] = i;
            heap.push(State { v: remain, cost: i });
        }
    }

    while let Some(State { v, cost }) = heap.pop() {
        if v == 0 {
            println!("{}", cost);
            return;
        }
        for i in 0..10 {
            let remain = (v * 10 + i) % k;
            if dist[remain] > dist[v] + i {
                dist[remain] = dist[v] + i;
                heap.push(State {
                    v: remain,
                    cost: dist[remain],
                });
            }
        }
    }
}
