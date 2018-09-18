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

use std::cmp;

fn main() {
    input!(n: usize, k: usize, a: [usize1; k], s: [chars; n]);

    let mut contained = vec![false; n];
    for &a in a.iter() {
        contained[a] = true;
    }

    let mut word: Vec<char> = s[a[0]].clone();
    for &a in a.iter() {
        let length = cmp::min(word.len(), s[a].len());
        for i in 0..length {
            if word[i] == s[a][i] {
                continue;
            }
            while word.len() > i {
                word.pop();
            }
            break;
        }
        while word.len() > length {
            word.pop();
        }
    }

    let mut required_length = 0;
    for i in 0..n {
        if contained[i] {
            continue;
        }
        let mut ok = false;
        if word.len() > s[i].len() {
            ok = true;
        }
        for j in 0..word.len() {
            if j >= s[i].len() || word[j] != s[i][j] {
                ok = true;
                required_length = cmp::max(required_length, j + 1);
                break;
            }
        }

        if !ok {
            println!("-1");
            return;
        }
    }

    for (i, &c) in word.iter().enumerate() {
        if i >= required_length {
            break;
        }
        print!("{}", c);
    }
    println!();
}
