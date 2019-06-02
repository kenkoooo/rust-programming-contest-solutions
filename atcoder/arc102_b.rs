fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let l: usize = sc.read();
    let mut r = 1;
    while (1 << (r + 1)) <= l {
        r += 1;
    }

    let n = r + 1;
    let mut graph = vec![vec![]; n + 1];
    for i in 0..r {
        graph[i].push((i + 1, 1 << i));
        graph[i].push((i + 1, 0));
    }

    let sum = (1 << r) - 1;
    let mut upper = l;
    for i in (0..r).rev() {
        let to_i = (1 << i) - 1;
        if upper <= to_i {
            continue;
        }
        let new_edge = upper - to_i - 1;
        // add [new_edge, ..., to_i+new_edge]
        if new_edge > sum {
            graph[i].push((n - 1, new_edge));
            upper = new_edge;
        }
    }

    let mut ans = vec![];
    for (from, v) in graph.into_iter().enumerate() {
        for (to, w) in v.into_iter() {
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
