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
    input!(n: usize, m: usize, uv: [(usize1, usize1); m]);
    let mut graph = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut q = VecDeque::new();
    let mut vis = vec![0; n];
    let mut components = vec![];
    for i in 0..n {
        if vis[i] != 0 {
            continue;
        }
        let mut is_bipartite = true;
        let mut c = vec![];
        q.push_back(i);
        vis[i] = 1;
        c.push(i);
        while let Some(v) = q.pop_front() {
            for &next in graph[v].iter() {
                if vis[next] != 0 {
                    if vis[next] == vis[v] {
                        is_bipartite = false;
                    }
                    continue;
                }
                vis[next] = if vis[v] == 1 { 2 } else { 1 };
                q.push_back(next);
                c.push(next);
            }
        }

        components.push((c, is_bipartite));
    }

    let mut cnt_one = 0;
    let mut cnt_bipartite = 0;
    let mut cnt_other = 0;
    for &(ref c, is_bipartite) in components.iter() {
        if c.len() == 1 {
            cnt_one += 1;
        } else if is_bipartite {
            cnt_bipartite += 1;
        } else {
            cnt_other += 1;
        }
    }

    let mut ans = 0;
    ans += cnt_one * n * 2 - cnt_one * cnt_one;
    ans += cnt_bipartite * 2;
    ans += cnt_other;
    ans += cnt_bipartite * (cnt_bipartite - 1) * 2;
    ans += cnt_other * (cnt_other - 1);
    ans += cnt_other * cnt_bipartite * 2;
    println!("{}", ans);
}
