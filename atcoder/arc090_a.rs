use std::cmp;

fn main() {
    let n = read_values::<usize>()[0];
    let a = (0..2).map(|_| read_values::<usize>()).collect::<Vec<_>>();
    let mut dp = vec![vec![0; n]; 2];
    dp[0][0] = a[0][0];
    for i in 1..n {
        dp[0][i] = dp[0][i - 1] + a[0][i];
    }
    dp[1][0] = dp[0][0] + a[1][0];
    for i in 1..n {
        dp[1][i] = cmp::max(dp[1][i - 1], dp[0][i]) + a[1][i];
    }

    println!("{}", dp[1][n - 1]);
}

fn read_line() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}
