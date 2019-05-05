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

use std::collections::VecDeque;

fn main() {
    input!(
        n: usize,
        uv: [(usize1, usize1); n],
        q: usize,
        ab: [(usize1, usize1); q]
    );

    let mut graph = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut queue = VecDeque::new();
    let mut visit_count = vec![0; n];
    let mut visit = vec![false; n];
    for i in 0..n {
        if graph[i].len() == 1 {
            queue.push_back(i);
            visit[i] = true;
        }
    }

    while let Some(v) = queue.pop_front() {
        for &next in graph[v].iter() {
            if visit[next] {
                continue;
            }
            visit_count[next] += 1;
            if visit_count[next] == graph[next].len() - 1 {
                visit[next] = true;
                queue.push_back(next);
            }
        }
    }

    for &(a, b) in ab.iter() {
        if visit[a] || visit[b] {
            println!("1");
        } else {
            println!("2");
        }
    }
}
