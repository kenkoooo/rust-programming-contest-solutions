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
use std::collections::{BTreeMap, BTreeSet};

fn t(x: usize, y: usize) -> (usize, usize) {
    if x > y {
        (y, x)
    } else {
        (x, y)
    }
}

fn rec(x: usize, y: usize, map: &mut BTreeMap<(usize, usize), usize>) -> usize {
    let state = t(x, y);
    if x < 2 && y < 2 {
        0
    } else if map.contains_key(&state) {
        *map.get(&state).unwrap()
    } else {
        let (x, y) = state;
        let mut set = BTreeSet::new();
        let mut i = 1;
        while 2 * i <= y {
            if 2 * i <= x {
                let tmp = rec(x - 2 * i, y + i, map);
                set.insert(tmp);
            }
            let tmp = rec(x + i, y - 2 * i, map);
            set.insert(tmp);
            i += 1;
        }

        let mut grundy = 0;
        for i in 0..(set.len() + 1) {
            if !set.contains(&i) {
                grundy = i;
                break;
            }
        }
        map.insert(state, grundy);
        grundy
    }
}

fn main() {
    input!(x: usize, y: usize);

    // let mut map = BTreeMap::new();
    // for x in 0..100 {
    //     for y in x..100 {
    //         rec(x, y, &mut map);
    //     }
    // }
    // for (key, &value) in map.iter() {
    //     if value == 0 {
    //         println!("{:?} {}", key, value);
    //     }
    // }
    let (x, y) = t(x, y);
    if x == y || x + 1 == y {
        println!("Brown");
    } else {
        println!("Alice");
    }
}
