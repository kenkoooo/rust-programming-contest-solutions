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

use std::collections::VecDeque;

fn main() {
    input!(n: usize, m: usize, s: chars, edges: [(usize1, usize1); m]);

    let s: Vec<usize> = s.iter().map(|&c| if c == 'A' { 0 } else { 1 }).collect();
    let mut graph = vec![vec![]; n];
    let mut count = vec![vec![0; 2]; n];
    for i in 0..m {
        let (from, to) = edges[i];
        graph[from].push(to);
        graph[to].push(from);
        count[from][s[to]] += 1;
        count[to][s[from]] += 1;
    }

    let mut vis = vec![false; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        if count[i][0] == 0 || count[i][1] == 0 {
            vis[i] = true;
            q.push_back(i);
        }
    }

    while let Some(i) = q.pop_front() {
        for &next in graph[i].iter() {
            if vis[next] {
                continue;
            }
            count[next][s[i]] -= 1;
            if count[next][s[i]] == 0 {
                vis[next] = true;
                q.push_back(next);
            }
        }
    }
    for &vis in vis.iter() {
        if !vis {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
