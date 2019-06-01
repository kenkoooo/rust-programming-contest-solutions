use std::cmp;

const INF: i64 = 1e15 as i64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let k: usize = sc.read();
    let v: Vec<i64> = sc.vec(n);
    let u: Vec<i64> = v.iter().rev().map(|&v| v).collect();

    let left_dp = get_table(v);
    let right_dp = get_table(u);

    let mut ans = -INF;
    for pop_left in 0..(n + 1) {
        for pop_right in 0..(n + 1) {
            if pop_left + pop_right > n {
                break;
            }
            for push_left in 0..(pop_left + 1) {
                for push_right in 0..(pop_right + 1) {
                    if pop_left + pop_right + push_left + push_right > k {
                        break;
                    }

                    let a = left_dp[pop_left][push_left] + right_dp[pop_right][push_right];
                    ans = cmp::max(ans, a);
                }
            }
        }
    }

    println!("{}", ans);
}

fn get_table(v: Vec<i64>) -> Vec<Vec<i64>> {
    let n = v.len();
    let mut dp = vec![vec![-INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for pop in 1..(n + 1) {
        let mut popped = vec![];
        let mut sum = 0;
        for i in 0..pop {
            popped.push(v[i]);
            sum += v[i];
        }
        dp[pop][0] = sum;
        popped.sort();
        for push in 1..(pop + 1) {
            let v = popped[push - 1];
            sum -= v;
            dp[pop][push] = sum;
        }
        assert_eq!(sum, 0);
    }
    dp
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
