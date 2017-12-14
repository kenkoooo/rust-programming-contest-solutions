use std::io;
use std::cmp;

fn main() {
    let n = read_values::<usize>()[0];
    let mut hp = (0..n).map(|_| {
        let v = read_values::<usize>();
        (v[0], v[1])
    }).collect::<Vec<_>>();
    hp.sort_by(|a, b| {
        let (h1, p1) = *a;
        let (h2, p2) = *b;
        (h1 + p1).cmp(&(h2 + p2))
    });

    let inf = 1e15 as usize;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let (h, p) = hp[i];
        for j in (0..n).rev() {
            if dp[j] <= h {
                dp[j + 1] = cmp::min(dp[j + 1], dp[j] + p);
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if dp[i + 1] < inf {
            ans = cmp::max(ans, i + 1);
        }
    }
    println!("{}", ans);
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}