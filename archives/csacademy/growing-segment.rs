use std::cmp;
use std::collections::BTreeSet;

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
    input!{
        n: usize,
        q: usize,
        x: [i64; n],
        l: [i64; q],
    }
    let mut l: Vec<(i64, usize)> = (0..q).map(|i| (l[i], i)).collect();
    l.sort();

    let mut x = {
        let mut v: Vec<i64> = vec![0];
        for &x in x.iter() {
            while v.len() >= 2 && (v[v.len() - 2] < v[v.len() - 1]) == (v[v.len() - 1] < x) {
                v.pop();
            }
            v.push(x);
        }
        v
    };

    let mut offset = 0;
    if x.len() >= 2 && x[1] < 0 {
        offset = -x[1];
        x.remove(0);
        for x in x.iter_mut() {
            *x += offset;
        }
    }

    let mut point_set = BTreeSet::new();
    for i in 0..x.len() {
        point_set.insert(i);
    }

    let mut edge_set = EdgeSet::new(offset);
    for i in 1..x.len() {
        edge_set.insert(i - 1, i, &x);
    }

    let mut ans = vec![0; q];
    for &(length, ans_id) in l.iter() {
        while let Some(&(distance, i1, i2)) = edge_set.first() {
            if distance >= length {
                break;
            }

            if edge_set.set.len() == 1 {
                break;
            }

            if i1 == 0 {
                let &i3 = point_set.range((i2 + 1)..).next().unwrap();

                edge_set.remove(i1, i2, &x);
                edge_set.remove(i2, i3, &x);

                x[i2] = length;
                edge_set.insert(i1, i2, &x);
                edge_set.insert(i2, i3, &x);
                continue;
            }

            let i3 = point_set.range((i2 + 1)..).next().cloned();
            if i3.is_none() {
                edge_set.remove(i1, i2, &x);
                point_set.remove(&i2);
                continue;
            }

            let &i0 = point_set.range(..i1).next_back().unwrap();
            let i3 = i3.unwrap();
            assert!((x[i0] < x[i1]) != (x[i1] < x[i2]));
            assert!((x[i1] < x[i2]) != (x[i2] < x[i3]));

            // remove i2
            edge_set.remove(i1, i2, &x);
            edge_set.remove(i2, i3, &x);
            point_set.remove(&i2);

            if (x[i0] - x[i1]).abs() > (x[i0] - x[i3]).abs() {
                // remove i3
                let &i4 = point_set.range((i3 + 1)..).next_back().unwrap();
                point_set.remove(&i3);
                edge_set.remove(i3, i4, &x);
                edge_set.insert(i1, i4, &x);
            } else {
                // remove i1
                point_set.remove(&i1);
                edge_set.remove(i0, i1, &x);
                edge_set.insert(i0, i3, &x);
            }
        }

        ans[ans_id] = cmp::max(0, edge_set.total - (edge_set.set.len() as i64) * length);
    }
    for &ans in ans.iter() {
        println!("{}", ans);
    }
}

#[derive(Debug)]
struct EdgeSet {
    set: BTreeSet<(i64, usize, usize)>,
    total: i64,
}

impl EdgeSet {
    fn new(offset: i64) -> Self {
        EdgeSet {
            set: BTreeSet::new(),
            total: offset,
        }
    }

    fn insert(&mut self, from: usize, to: usize, x: &Vec<i64>) {
        assert!(from < to);
        let distance = (x[from] - x[to]).abs();
        self.set.insert((distance, from, to));
        self.total += distance;
    }

    fn remove(&mut self, from: usize, to: usize, x: &Vec<i64>) {
        assert!(from < to);
        let distance = (x[from] - x[to]).abs();
        self.set.remove(&(distance, from, to));
        self.total -= distance;
    }

    fn first(&self) -> Option<&(i64, usize, usize)> {
        self.set.iter().next()
    }
}
