use std::cmp::max;

const COST: [usize; 9] = [2, 5, 5, 4, 5, 6, 3, 7, 6];
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut dp = vec![0; n + 1];
    let mut a = (0..m).map(|_| sc.read::<usize>() - 1).collect::<Vec<_>>();
    a.sort();
    a.reverse();
    for used in 0..n {
        if used > 0 && dp[used] == 0 {
            continue;
        }
        for &a in a.iter() {
            let cost = COST[a];
            if used + cost <= n {
                dp[used + cost] = max(dp[used + cost], dp[used] + 1);
            }
        }
    }

    let mut cur = n;
    while cur > 0 {
        for &a in a.iter() {
            let cost = COST[a];
            if cur >= cost && dp[cur - cost] + 1 == dp[cur] {
                if cur - cost > 0 && dp[cur - cost] == 0 {
                    continue;
                }
                print!("{}", a + 1);
                cur -= cost;
                break;
            }
        }
    }
    println!();
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
