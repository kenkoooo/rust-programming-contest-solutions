use std::cmp;
use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let r: usize = sc.read();
    let p: Vec<usize> = sc.vec::<usize>(n).into_iter().map(|x| x - 1).collect();

    let mut used = vec![0; n];
    let mut cur = 1;
    for i in 0..n {
        if used[i] != 0 {
            continue;
        }
        let mut pos = i;
        while used[pos] == 0 {
            used[pos] = cur;
            pos = p[pos];
        }
        cur += 1;
    }

    let count = used
        .into_iter()
        .fold(BTreeMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        })
        .into_iter()
        .fold(BTreeMap::new(), |mut map, (_, count)| {
            *map.entry(count).or_insert(0) += 1;
            map
        });

    let r = cmp::min(r, n - r);
    let mut dp = vec![-1; r + 1];
    dp[0] = 0;
    for (a, m) in count.into_iter() {
        for j in 0..(r + 1) {
            if dp[j] >= 0 {
                dp[j] = m;
            } else if j < a || dp[j - a] <= 0 {
                dp[j] = -1;
            } else {
                dp[j] = dp[j - a] - 1;
            }
        }
    }

    if dp[r] >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
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
