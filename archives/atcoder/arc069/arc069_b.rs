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

fn solve(s: &Vec<char>, first: bool, second: bool) -> Option<Vec<bool>> {
    let n = s.len();
    let mut is_wolf = vec![false; n];
    is_wolf[0] = first;
    is_wolf[1] = second;
    for i in 1..(n - 1) {
        let prev = i - 1;
        let next = i + 1;
        match (s[i], is_wolf[i]) {
            ('o', true) => {
                is_wolf[next] = !is_wolf[prev];
            }
            ('x', true) => {
                is_wolf[next] = is_wolf[prev];
            }
            ('o', false) => {
                is_wolf[next] = is_wolf[prev];
            }
            ('x', false) => {
                is_wolf[next] = !is_wolf[prev];
            }
            _ => unreachable!(),
        }
    }
    let honest = (s[n - 1] == 'o') == (is_wolf[n - 2] == is_wolf[0]);
    if (!honest) != is_wolf[n - 1] {
        return None;
    }

    let honest = (s[0] == 'o') == (is_wolf[n - 1] == is_wolf[1]);
    if (!honest) != is_wolf[0] {
        return None;
    }
    Some(is_wolf)
}

fn main() {
    input!(n: usize, s: chars);
    if let Some(is_wolf) = solve(&s, true, true) {
        for &is_wolf in is_wolf.iter() {
            print!("{}", if is_wolf { "W" } else { "S" });
        }
        println!();
    } else if let Some(is_wolf) = solve(&s, true, false) {
        for &is_wolf in is_wolf.iter() {
            print!("{}", if is_wolf { "W" } else { "S" });
        }
        println!();
    } else if let Some(is_wolf) = solve(&s, false, true) {
        for &is_wolf in is_wolf.iter() {
            print!("{}", if is_wolf { "W" } else { "S" });
        }
        println!();
    } else if let Some(is_wolf) = solve(&s, false, false) {
        for &is_wolf in is_wolf.iter() {
            print!("{}", if is_wolf { "W" } else { "S" });
        }
        println!();
    } else {
        println!("-1");
    }
}
