use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let q: usize = sc.read();
    let mut tree = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        tree[a].push(b);
        tree[b].push(a);
    }
    let mut add = vec![0; n];
    for _ in 0..q {
        let p = sc.read::<usize>() - 1;
        let x: u64 = sc.read();
        add[p] += x;
    }

    let mut ans = vec![0; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    ans[0] = add[0];
    while let Some((v, p)) = q.pop_front() {
        for &next in tree[v].iter() {
            if next == p {
                continue;
            }
            ans[next] = add[next] + ans[v];
            q.push_back((next, v));
        }
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
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
