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

use std::collections::{BTreeSet, VecDeque};

fn main() {
    input!(l: usize);

    let graph = solve(l);
    validate(&graph, l);

    let num = graph.iter().map(|edges| edges.len()).sum::<usize>();
    println!("{} {}", graph.len(), num);

    for from in 0..graph.len() {
        for &(to, cost) in graph[from].iter() {
            println!("{} {} {}", from + 1, to + 1, cost);
        }
    }
}

fn validate(graph: &Vec<Vec<(usize, usize)>>, l: usize) {
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let goal = graph.len() - 1;
    let mut set = BTreeSet::new();
    while let Some((v, dist)) = q.pop_front() {
        if v == goal {
            assert!(!set.contains(&dist));
            assert!(dist < l);
            set.insert(dist);
            continue;
        }
        for &(next, cost) in graph[v].iter() {
            q.push_back((next, cost + dist));
        }
    }
    assert!(set.len() == l);
}

fn solve(l: usize) -> Vec<Vec<(usize, usize)>> {
    let mut r = 0;
    while (1 << (r + 1)) <= l {
        r += 1;
    }
    assert!(r <= 19);
    let mut graph = vec![vec![]; r + 1];
    for i in 0..r {
        graph[i].push((i + 1, 1 << i));
        graph[i].push((i + 1, 0));
    }

    let n = graph.len();
    if (1 << (n - 1)) - 1 == l - 1 {
        graph
    } else {
        let mut l = l;
        for i in (0..(n - 1)).rev() {
            if l >= (1 << r) + (1 << i) {
                graph[i].push((n - 1, l - (1 << i)));
                l -= 1 << i;
            }
        }

        graph
    }
}
