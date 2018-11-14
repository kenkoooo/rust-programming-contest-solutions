use std::cmp;

fn main() {
    let stdin = std::io::stdin();
    let mut sc = scanner::Scanner::new(stdin.lock());
    let n: usize = sc.read();
    let c: i64 = sc.read();
    let mut x: Vec<i64> = vec![0; n];
    let mut v: Vec<i64> = vec![0; n];
    for i in 0..n {
        x[i] = sc.read();
        v[i] = sc.read();
    }

    let mut right_sum = vec![0; n + 1];
    for i in 0..n {
        right_sum[i + 1] = right_sum[i] + v[i];
    }

    let mut left_sum = vec![0; n + 1];
    for i in 0..n {
        left_sum[i + 1] = left_sum[i] + v[n - 1 - i];
    }

    let mut right_max = vec![0; n + 1];
    for i in 0..n {
        right_max[i + 1] = cmp::max(right_max[i], right_sum[i + 1] - x[i]);
    }

    let mut left_max = vec![0; n + 1];
    for i in 0..n {
        left_max[i + 1] = cmp::max(left_max[i], left_sum[i + 1] - (c - x[n - 1 - i]));
    }

    let mut ans = 0;
    for i in 0..n {
        let dist = c - x[n - 1 - i];
        let left = left_sum[i + 1] - dist * 2;
        ans = cmp::max(ans, left + right_max[n - 1 - i]);
    }
    for i in 0..n {
        let dist = x[i];
        let right = right_sum[i + 1] - dist * 2;
        ans = cmp::max(ans, right + left_max[n - 1 - i]);
    }
    ans = cmp::max(ans, left_max[n]);
    ans = cmp::max(ans, right_max[n]);
    println!("{}", ans);
}

pub mod scanner {
    use std::fmt::Debug;
    use std::io::Read;
    use std::str::{self, FromStr};

    pub struct Scanner<R: Read> {
        reader: R,
    }

    impl<R: Read> Scanner<R> {
        pub fn new(reader: R) -> Self {
            Scanner { reader: reader }
        }

        pub fn read<T>(&mut self) -> T
        where
            T: FromStr,
            T::Err: Debug,
        {
            let buf = self
                .reader
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n')
                .take_while(|&b| b != b' ' && b != b'\n')
                .collect::<Vec<_>>();

            unsafe { str::from_utf8_unchecked(&buf) }
                .parse()
                .expect("Parse error.")
        }
    }
}
