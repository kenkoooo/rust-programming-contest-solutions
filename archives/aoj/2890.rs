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
    input!(n: usize, q: usize, a: [usize1; n], q: [usize1; q]);
    if n == 1 {
        println!("{}", a[0] + 1);
        return;
    }
    let mut left = vec![0; n + 2];
    let mut right = vec![0; n + 2];

    let head = n;
    let tail = n + 1;

    for i in 0..n {
        let l = if i == 0 { head } else { a[i - 1] };
        let r = if i == n - 1 { tail } else { a[i + 1] };
        left[a[i]] = l;
        right[a[i]] = r;
    }
    right[head] = a[0];
    left[tail] = a[n - 1];

    for &q in q.iter() {
        if right[head] == q {
            assert_eq!(left[q], head);
            let right_q = right[q];
            let left_tail = left[tail];

            left[tail] = q;
            right[q] = tail;

            left[right_q] = head;
            right[head] = right_q;
            right[left_tail] = q;
            left[q] = left_tail;
        } else if left[tail] == q {
            assert_eq!(right[q], tail);
            let right_head = right[head];
            let left_q = left[q];

            right[head] = q;
            left[q] = head;

            left[right_head] = q;
            right[q] = right_head;

            left[tail] = left_q;
            right[left_q] = tail;
        } else {
            let right_head = right[head];
            let left_q = left[q];
            let right_q = right[q];
            let left_tail = left[tail];

            left[right_head] = q;
            right[q] = right_head;
            right[left_q] = tail;
            left[tail] = left_q;

            right[head] = right_q;
            left[right_q] = head;
            right[left_tail] = q;
            left[q] = left_tail;
        }
    }

    let mut cur = right[head];
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", cur + 1);
        cur = right[cur];
    }
    println!();
    assert_eq!(cur, tail);
}
