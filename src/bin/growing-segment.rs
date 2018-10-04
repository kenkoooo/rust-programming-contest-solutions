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

fn main() {
    input!(n: usize, q: usize, x: [i64; n], l: [i64; q]);

    let mut v: Vec<i64> = vec![0];
    for &x in x.iter() {
        while v.len() >= 2 && (v[v.len() - 2] < v[v.len() - 1]) == (v[v.len() - 1] < x) {
            v.pop();
        }
        v.push(x);
    }

    let mut total = 0;
    if v.len() >= 2 && v[1] < 0 {
        let t = -v[1];
        v.remove(0);
        for v in v.iter_mut() {
            *v += t;
        }
        total += t;
    }

    let n = v.len();
    let x = v;

    let mut set = BTreeSet::new();
    let mut points = BTreeSet::new();
    for i in 0..(x.len() - 1) {
        let d = (x[i + 1] - x[i]).abs();
        total += d;
        set.insert((d, i));
        points.insert(i);
    }
    points.insert(x.len() - 1);

    let mut ans = vec![0; q];
    let mut queries = l
        .iter()
        .enumerate()
        .map(|(i, &l)| (l, i))
        .collect::<Vec<_>>();
    queries.sort();

    let mut prev_l = 0;
    for &(l, q) in queries.iter() {
        while let Some(&(d, i)) = set.iter().next() {
            if d > l {
                break;
            }

            let p1 = i;
            let p2 = *points.range((p1 + 1)..).next().unwrap();

            points.remove(&p2);
            set.remove(&(d, p1));
            total -= d - prev_l;

            let p3 = points.range((p2 + 1)..).next().cloned();
            if p3.is_none() {
                continue;
            }

            let p3 = p3.unwrap();
            let d = (x[p2] - x[p3]).abs();
            set.remove(&(d, p2));
            total -= d - prev_l;

            let p0 = points.range(..p1).next_back().cloned();
            if p0.is_none() {
                points.remove(&p1);

                assert!(x[p3] <= x[p1]);
                total += x[p1] - x[p3];
                continue;
            }

            let p0 = p0.unwrap();
            if (x[p0] - x[p1]).abs() < (x[p0] - x[p3]).abs() {
                points.remove(&p1);

                let d = (x[p0] - x[p1]).abs();
                set.remove(&(d, p0));
                total -= d - prev_l;

                let d = (x[p0] - x[p3]).abs();
                set.insert((d, p0));
                total += d - prev_l;
            } else {
                points.remove(&p3);

                if let Some(&p4) = points.range((p3 + 1)..).next() {
                    let d = (x[p3] - x[p4]).abs();
                    set.remove(&(d, p3));
                    total -= d - prev_l;

                    let d = (x[p1] - x[p4]).abs();
                    set.insert((d, p1));
                    total += d - prev_l;
                }
            }
        }

        total -= (l - prev_l) * (set.len() as i64);
        ans[q] = total;
        prev_l = l;
    }

    for &ans in ans.iter() {
        println!("{}", ans);
    }
}
