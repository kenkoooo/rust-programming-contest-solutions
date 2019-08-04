fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();

    let mut x: Vec<i64> = vec![];
    x.push(sc.read());
    for _ in 1..n {
        let tail = *x.iter().next_back().unwrap();
        let a = sc.read::<i64>();
        if a != tail {
            x.push(a);
        }
    }
    let n = x.len();
    if n == 1 {
        println!("0");
        return;
    }

    let mut increasing = x[1] > x[0];
    let mut ans = vec![x[0], x[1]];
    for i in 2..n {
        let tail = ans.len() - 1;
        if (increasing && x[i] > ans[tail]) || (!increasing && x[i] < ans[tail]) {
            ans[tail] = x[i];
        } else {
            ans.push(x[i]);
            increasing = !increasing;
        }
    }

    println!("{}", ans.len());
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
