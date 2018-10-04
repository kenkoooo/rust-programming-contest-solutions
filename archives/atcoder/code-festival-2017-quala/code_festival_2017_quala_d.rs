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
    input!(h: usize, w: usize, d: i64);
    let max_d = d * ((h * w) as i64);

    let mut ans = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            let i = i as i64;
            let j = j as i64;
            let x = i + j;
            let y = i - j;

            let bi = x / d;
            let bj = if y < 0 { (y - d + 1) / d } else { y / d };
            let color = (bi % 2) * 2 + if bj < 0 { (-bj) % 2 } else { bj % 2 };

            ans[i as usize][j as usize] = color as usize;
        }
    }

    let c: Vec<char> = "RYGB".chars().collect();

    for i in 0..h {
        for j in 0..w {
            print!("{}", c[ans[i][j]]);
        }
        println!();
    }
}
