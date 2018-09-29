const M: usize = 1e9 as usize + 7;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s: Vec<usize> = s
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    let (n, d) = (s[0], s[1]);
    let mut c = [[0; 31]; 31];
    for i in 0..n {
        c[i][0] = 1;
        for j in 1..(i + 1) {
            c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) % M;
        }
    }

    let mut pow2 = [0; 999];
    pow2[0] = 1;
    for i in 1..pow2.len() {
        pow2[i] = (pow2[i - 1] * 2) % M;
    }

    let mut dp = [[[0; 31]; 31]; 30];
    dp[0][1][1] = 1;

    for dist in 1..n {
        for used in 1..(n + 1) {
            for next in 1..n {
                if used + next > n {
                    continue;
                }
                for last_used in 1..(used + 1) {
                    let unused = if dist <= d { n - used - 1 } else { n - used };
                    let selecting = if dist == d { next - 1 } else { next };
                    if unused < selecting {
                        continue;
                    }

                    let mut v = c[unused][selecting];
                    for _ in 0..next {
                        v *= (1 << last_used) - 1;
                        v %= M;
                    }

                    v *= pow2[next * (next - 1) / 2];
                    v %= M;

                    dp[dist][used + next][next] += (dp[dist - 1][used][last_used] * v) % M;
                    dp[dist][used + next][next] %= M;
                }
            }
        }
    }

    let mut ans = 0;
    for dist in d..n {
        for used in 1..(n + 1) {
            let mut count = 0;
            for last in 1..(used + 1) {
                count += dp[dist][used][last];
                count %= M;
            }

            let unused = n - used;
            let unconnected = if unused > 0 {
                pow2[unused * (unused - 1) / 2]
            } else {
                1
            };
            ans += (count * unconnected) % M;
            ans %= M;
        }
    }

    println!("{}", ans);
}
