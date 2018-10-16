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
    input!(n: usize, a: [i64; n]);
    let mut init_check = true;
    for i in 1..n {
        if a[i] < a[i - 1] {
            init_check = false;
            break;
        }
    }
    if init_check {
        println!("0");
        return;
    }

    let mut max_i = 0;
    let mut min_i = 0;
    for i in 0..n {
        if a[i] > a[max_i] {
            max_i = i;
        }
        if a[i] < a[min_i] {
            min_i = i;
        }
    }

    let mut a = a;
    let mut ans = vec![];
    if a[max_i].abs() >= a[min_i].abs() {
        assert!(a[max_i] > 0);
        for i in 0..n {
            if a[i] < 0 {
                a[i] += a[max_i];
                ans.push((i, max_i));
            }
        }
        for i in 1..n {
            a[i] += a[i - 1];
            ans.push((i, i - 1));
            assert!(a[i - 1] <= a[i]);
        }
    } else {
        assert!(a[min_i] < 0);
        for i in 0..n {
            if a[i] > 0 {
                a[i] += a[min_i];
                ans.push((i, min_i));
            }
        }
        for i in (1..n).rev() {
            a[i - 1] += a[i];
            ans.push((i - 1, i));
            assert!(a[i - 1] <= a[i]);
        }
    }

    println!("{}", ans.len());
    for &(y, x) in ans.iter() {
        println!("{} {}", x + 1, y + 1);
    }
}
