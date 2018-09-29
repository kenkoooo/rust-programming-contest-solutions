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
use std::collections::BTreeSet;

fn main() {
    input!(s: chars);
    let mut can_build = vec![true; s.len() + 1];
    for i in 0..s.len() {
        can_build[i + 1] = s[i] == '1';
    }

    match solve(&can_build) {
        Some(result) => for &(a, b) in result.iter() {
            println!("{} {}", a, b);
        },
        None => {
            println!("-1");
        }
    }
}

fn solve(can_build: &Vec<bool>) -> Option<Vec<(usize, usize)>> {
    let n = can_build.len() - 1;
    if !can_build[1] || can_build[n] {
        return None;
    }

    let mut v = vec![];
    for i in 1..n {
        if can_build[i] {
            v.push(i);
        }
    }

    for i in 0..v.len() {
        if v[i] + v[v.len() - 1 - i] != n {
            return None;
        }
    }
    assert_eq!(v[0], 1);
    if v.len() == 2 {
        let mut result = vec![];
        for i in 1..n {
            result.push((1, i + 1));
        }
        return Some(result);
    }

    let m = v.len();
    let mut graph = vec![vec![]; n];
    graph[0].push(1);
    graph[1].push(0);
    let mut tail = n - 1;
    for i in 1..m {
        graph[i].push(i + 1);
        graph[i + 1].push(i);
        let required = v[i] - v[i - 1] - 1;
        for _ in 0..required {
            graph[i].push(tail);
            graph[tail].push(i);
            tail -= 1;
            if tail == 0 {
                return None;
            }
        }
        if v[i] * 2 == n {
            for _ in 1..v[i] {
                if tail == i {
                    return None;
                }
                graph[i + 1].push(tail);
                graph[tail].push(i + 1);
                tail -= 1;
            }
            break;
        }
        if v[i] + v[i + 1] == n {
            for _ in 1..v[i + 1] {
                if tail == i {
                    return None;
                }
                graph[i + 1].push(tail);
                graph[tail].push(i + 1);
                tail -= 1;
            }
            break;
        }
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        for &j in graph[i].iter() {
            assert!(i != j);
            let x = cmp::min(i, j);
            let y = cmp::max(i, j);
            set.insert((x, y));
        }
    }
    Some(set.iter().map(|&(x, y)| (x + 1, y + 1)).collect())
}
