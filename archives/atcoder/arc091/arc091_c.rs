use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let a: usize = sc.read();
    let b: usize = sc.read();
    if a + b > n + 1 || a < (n + b - 1) / b {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    let mut cur = 0;
    if n % b != 0 {
        ans.push(vec![]);
        for _ in 0..(n % b) {
            ans[0].push(cur);
            cur += 1;
        }
        ans[0].reverse();
    }
    for _ in 0..(n / b) {
        let mut lds = vec![];
        for _ in 0..b {
            lds.push(cur);
            cur += 1;
        }
        lds.reverse();
        ans.push(lds);
    }
    assert_eq!(cur, n);

    let mut a = a - ans.len();
    for i in 0..ans.len() {
        if a > 0 {
            let len = cmp::min(a + 1, ans[i].len());
            ans[i][0..len].sort();
            a -= len - 1;
        }
    }

    for ans in ans.iter() {
        for &ans in ans.iter() {
            print!("{} ", ans + 1);
        }
    }
    println!();
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
