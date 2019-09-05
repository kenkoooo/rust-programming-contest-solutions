use std::cmp::max;

const COSTS: [usize; 10] = [1000, 2, 5, 5, 4, 5, 6, 3, 7, 6];

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut a: Vec<usize> = sc.vec(m);
    a.sort();
    a.reverse();

    let mut dp = vec![-1; n + 1];
    dp[0] = 0;
    for i in 0..n {
        if dp[i] < 0 {
            continue;
        }
        for &a in a.iter() {
            let cost = COSTS[a];
            if i + cost > n {
                continue;
            }
            dp[i + cost] = max(dp[i + cost], dp[i] + 1);
        }
    }

    let mut cur = n;
    while cur > 0 {
        for &a in a.iter() {
            let cost = COSTS[a];
            if cur >= cost && dp[cur] > 0 && dp[cur - cost] == dp[cur] - 1 {
                print!("{}", a);
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
