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
    input!(a: usize, b: usize);
    let h = 100;
    let w = 100;
    let mut is_white = vec![vec![false; w]; h];

    let mut white = a - 1;
    let mut black = b - 1;

    for i in 0..h {
        for j in 0..w {
            if i <= h / 2 {
                if i % 2 == 1 && j % 2 == 0 {
                    is_white[i][j] = false;
                    if white > 0 {
                        is_white[i][j] = true;
                        white -= 1;
                    }
                } else {
                    is_white[i][j] = false;
                }
            } else {
                if i % 2 == 0 && j % 2 == 0 {
                    is_white[i][j] = true;
                    if black > 0 {
                        is_white[i][j] = false;
                        black -= 1;
                    }
                } else {
                    is_white[i][j] = true;
                }
            }
        }
    }

    println!("{} {}", h, w);

    for i in 0..h {
        for j in 0..w {
            print!("{}", if is_white[i][j] { '.' } else { '#' });
        }
        println!();
    }
}
