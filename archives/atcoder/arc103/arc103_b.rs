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

fn main() {
    input!(n: usize, xy: [(i64, i64); n]);
    let m = ((xy[0].0).abs() + (xy[0].1).abs()) & 1;
    for &(x, y) in xy.iter() {
        if m != (x.abs() + y.abs()) & 1 {
            println!("-1");
            return;
        }
    }
    let mut edges: Vec<i64> = vec![];
    if m == 0 {
        edges.push(1);
    }
    for i in 0..32 {
        edges.push(1 << i);
    }

    let m = edges.len();
    println!("{}", m);
    for i in 0..m {
        if i > 0 {
            print!(" ");
        }
        print!("{}", edges[i]);
    }
    println!();

    let sum = edges.iter().sum::<i64>();

    for &(x, y) in xy.iter() {
        let u = x + y;
        let mut arms_u = vec![false; m];
        let v = x - y;
        let mut arms_v = vec![false; m];

        let u = u + sum;
        let v = v + sum;
        let mut cur_u = 0;
        let mut cur_v = 0;
        for i in (0..m).rev() {
            let edge = edges[i];
            if cur_u + 2 * edge <= u {
                cur_u += 2 * edge;
                arms_u[i] = true;
            }
            if cur_v + 2 * edge <= v {
                cur_v += 2 * edge;
                arms_v[i] = true;
            }
        }
        assert_eq!(cur_u, u);
        assert_eq!(cur_v, v);

        let mut ans: Vec<char> = vec![];
        for i in 0..m {
            match (arms_u[i], arms_v[i]) {
                (true, true) => {
                    ans.push('R');
                }
                (true, false) => {
                    ans.push('U');
                }
                (false, true) => {
                    ans.push('D');
                }
                (false, false) => {
                    ans.push('L');
                }
            }
        }

        validate(&ans, &edges, x, y);
        for &c in ans.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn validate(s: &Vec<char>, edges: &Vec<i64>, x: i64, y: i64) {
    let mut cur_x = 0;
    let mut cur_y = 0;
    assert_eq!(s.len(), edges.len());
    let n = s.len();
    for i in 0..n {
        match s[i] {
            'R' => {
                cur_x += edges[i];
            }
            'L' => {
                cur_x -= edges[i];
            }
            'D' => {
                cur_y -= edges[i];
            }
            'U' => {
                cur_y += edges[i];
            }
            _ => unreachable!(),
        }
    }
    assert_eq!((cur_x, cur_y), (x, y));
}
