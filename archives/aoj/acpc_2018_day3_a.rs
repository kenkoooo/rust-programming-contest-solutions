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

fn main() {
    input!(s: chars);

    let n = s.len();
    let mut ans = 0;
    for i in 1..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let s1: String = s[0..i].iter().map(|&c| c).collect();
                let s2: String = s[i..j].iter().map(|&c| c).collect();
                let s3: String = s[j..k].iter().map(|&c| c).collect();
                let s4: String = s[k..].iter().map(|&c| c).collect();
                assert!(s1.len() > 0);
                assert!(s2.len() > 0);
                assert!(s3.len() > 0);
                assert!(s4.len() > 0);

                let t1 = s1.parse::<usize>().unwrap();
                let t2 = s2.parse::<usize>().unwrap();
                let t3 = s3.parse::<usize>().unwrap();
                let t4 = s4.parse::<usize>().unwrap();
                if t1 > 255 || t2 > 255 || t3 > 255 || t4 > 255 {
                    continue;
                }

                let t1 = t1.to_string();
                let t2 = t2.to_string();
                let t3 = t3.to_string();
                let t4 = t4.to_string();
                if t1.len() != s1.len()
                    || t2.len() != s2.len()
                    || t3.len() != s3.len()
                    || t4.len() != s4.len()
                {
                    continue;
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
