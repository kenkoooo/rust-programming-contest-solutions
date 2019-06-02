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

fn rotate(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = s.len();
    let m = s[0].len();
    let mut next = vec![vec!['a'; n]; m];
    for i in 0..m {
        for j in 0..n {
            next[i][j] = s[n - 1 - j][i];
        }
    }
    next
}

fn calc(s: &Vec<Vec<char>>) -> usize {
    let n = s.len();
    let m = s[0].len();
    let mut calc1 = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 1..m {
            if s[i][j] != '#' {
                calc1[i][j] = calc1[i][j - 1];
                if s[i][j - 1] != '#' {
                    calc1[i][j] += 1;
                }
            }
        }
    }
    let mut calc2 = vec![vec![0; m]; n];
    for i in 1..n {
        for j in 0..m {
            if s[i][j] != '#' {
                calc2[i][j] = calc2[i - 1][j] + calc1[i - 1][j];
            }
        }
    }
    calc2.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>()
}

fn main() {
    input!(n: usize, m: usize, s: [chars; n]);
    let ans1 = calc(&s);

    let s = rotate(&s);
    let ans2 = calc(&s);

    let s = rotate(&s);
    let ans3 = calc(&s);

    let s = rotate(&s);
    let ans4 = calc(&s);
    println!("{}", ans1 + ans2 + ans3 + ans4);
}
