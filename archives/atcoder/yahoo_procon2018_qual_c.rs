use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let x = (0..n).map(|_| sc.read::<usize>()).collect::<Vec<_>>();
    let c = (0..n).map(|_| sc.read::<usize>()).collect::<Vec<_>>();
    let v = (0..n).map(|_| sc.read::<usize>()).collect::<Vec<_>>();

    let mut money = vec![0; n + 1];
    for i in 0..n {
        money[i + 1] = money[i] + x[i];
    }

    let mut total_costs = vec![0; 1 << n];
    let mut total_values = vec![0; 1 << n];
    for mask in 0..(1 << n) {
        for i in 0..n {
            let prohibited = (mask & (1 << i)) != 0;
            if prohibited {
                continue;
            }
            total_costs[mask] += c[i];
            total_values[mask] += v[i];
        }
    }

    let mut dp = vec![vec![std::usize::MAX; 1 << n]; n + 1];

    for k in 0..(n + 1) {
        maximize(0, n, k, money[k], &total_costs, &mut dp[k], &total_values);
    }

    let mut memo = vec![std::usize::MAX; 1 << n];
    println!("{}", minimize(0, n, 0, &dp, &mut memo));
}

fn minimize(prohibited_mask: usize, n: usize, k: usize, dp: &Vec<Vec<usize>>, memo: &mut Vec<usize>) -> usize {
    if k == n { return 0; }
    if memo[prohibited_mask] != std::usize::MAX { return memo[prohibited_mask]; }
    let mut x = std::usize::MAX;
    for i in 0..n {
        if (prohibited_mask & (1 << i)) == 0 {
            x = cmp::min(minimize(prohibited_mask | (1 << i), n, k + 1, dp, memo), x);
        }
    }

    if x == std::usize::MAX { x = 0; }
    memo[prohibited_mask] = cmp::max(dp[k][prohibited_mask], x);
    return memo[prohibited_mask];
}

fn maximize(prohibited_mask: usize, n: usize, k: usize, money: usize, total_costs: &Vec<usize>, dp: &mut Vec<usize>, total_values: &Vec<usize>) -> usize {
    if dp[prohibited_mask] != std::usize::MAX {
        return dp[prohibited_mask];
    }

    let mut max = 0;
    if total_costs[prohibited_mask] <= money {
        max = total_values[prohibited_mask];
    }

    for i in 0..n {
        if (prohibited_mask & (1 << i)) == 0 {
            max = cmp::max(maximize(prohibited_mask | (1 << i), n, k, money, total_costs, dp, total_values), max);
        }
    }
    dp[prohibited_mask] = max;
    return max;
}

struct Scanner {
    buffer: std::collections::VecDeque<String>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { buffer: std::collections::VecDeque::new() }
    }

    fn read_line() -> String {
        let stdin = std::io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf
    }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        if self.buffer.is_empty() {
            for s in Scanner::read_line().split(' ') {
                self.buffer.push_back(s.to_string());
            }
        }
        self.buffer.pop_front().unwrap().trim().parse().unwrap()
    }
}
