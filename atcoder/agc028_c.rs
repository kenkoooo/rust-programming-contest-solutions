use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut edges = vec![];

    let mut left = 0;
    let mut right = 0;
    for i in 0..n {
        let a: u64 = sc.read();
        let b: u64 = sc.read();
        left += a;
        right += b;
        edges.push((a, i, true));
        edges.push((b, i, false));
    }
    edges.sort();

    let mut ans = cmp::min(left, right);

    let mut count = vec![0; n];
    let mut sum = 0;
    for i in 0..n {
        let (cost, i, _) = edges[i];
        count[i] += 1;
        sum += cost;
    }

    if count.iter().any(|&x| x != 1) {
        println!("{}", cmp::min(ans, sum));
        return;
    }

    if edges[n - 1].1 != edges[n].1 {
        sum -= edges[n - 1].0;
        sum += edges[n].0;
        println!("{}", cmp::min(ans, sum));
        return;
    }

    ans = cmp::min(ans, sum - edges[n - 2].0 + edges[n].0);
    ans = cmp::min(ans, sum - edges[n - 1].0 + edges[n + 1].0);
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
