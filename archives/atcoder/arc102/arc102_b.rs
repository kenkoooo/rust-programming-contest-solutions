fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let l: usize = sc.read();

    let mut r = 0;
    while (1 << (r + 1)) <= l {
        r += 1;
    }

    let initial_length = 1 << r;
    let n = r + 1;
    let mut graph = vec![vec![]; n];
    for i in 0..r {
        graph[i].push((i + 1, 1 << i));
        graph[i].push((i + 1, 0));
    }

    let mut l = l;
    for i in (0..(n - 1)).rev() {
        if l >= initial_length + (1 << i) {
            let t = l - (1 << i);
            graph[i].push((n - 1, t));
            l = t;
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        for &(next, w) in graph[i].iter() {
            ans.push((i, next, w));
        }
    }

    println!("{} {}", n, ans.len());
    for &(i, next, w) in ans.iter() {
        println!("{} {} {}", i + 1, next + 1, w);
    }
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
