fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let l: usize = sc.read();
    let mut n = 0;
    while (1 << (n + 1)) <= l {
        n += 1;
    }

    n += 1;
    let mut graph = vec![vec![]; n];
    for i in 1..n {
        graph[i - 1].push((i, 1 << (i - 1)));
        graph[i - 1].push((i, 0));
    }

    let lower = (1 << (n - 1)) - 1;
    let mut upper = l - 1;
    for i in (0..(n - 1)).rev() {
        let sum = (1 << i) - 1;
        if upper <= sum {
            continue;
        }
        let edge = upper - sum;

        // sum + edge == upper
        // 0 + edge
        if edge <= lower {
            continue;
        }
        upper = edge - 1;
        graph[i].push((n - 1, edge));
    }

    let mut ans = vec![];
    for from in 0..n {
        for &(to, w) in graph[from].iter() {
            ans.push((from, to, w));
        }
    }

    println!("{} {}", n, ans.len());
    for (from, to, w) in ans.into_iter() {
        println!("{} {} {}", from + 1, to + 1, w);
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
