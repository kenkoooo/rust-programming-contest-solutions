use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut tasks = vec![];
    for _ in 0..n {
        let days: usize = sc.read();
        let value: u64 = sc.read();
        tasks.push((value, days));
    }
    tasks.sort();
    tasks.reverse();
    let mut remain = (0..(m + 1)).collect::<BTreeSet<_>>();
    let mut ans = 0;
    for (value, days) in tasks.into_iter() {
        if m < days {
            continue;
        }
        let d = m - days;
        if let Some(&d) = remain.range(..(d + 1)).next_back() {
            remain.remove(&d);
            ans += value;
        }
    }
    println!("{}", ans);
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
