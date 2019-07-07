use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let b: Vec<usize> = sc.vec(n);

    if !check(50, &a, &b, &vec![]) {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    for k in (1..51).rev() {
        if !check(k - 1, &a, &b, &ans) {
            ans.push(k);
        }
    }

    println!("{}", ans.into_iter().map(|i| 1 << i).sum::<usize>());
}

fn check(k: usize, a: &Vec<usize>, b: &Vec<usize>, ans: &Vec<usize>) -> bool {
    let max = cmp::max(*a.iter().max().unwrap(), *b.iter().max().unwrap());
    let mut w = vec![vec![false; max + 1]; max + 1];
    for &a in a.iter() {
        w[a][a] = true;
    }

    for &k in ans.iter() {
        for i in 0..(max + 1) {
            w[i][i % k] = true;
        }
    }
    for k in 1..(k + 1) {
        for i in 0..(max + 1) {
            w[i][i % k] = true;
        }
    }
    for k in 0..(max + 1) {
        w[k][k] = true;
    }

    for k in 0..(max + 1) {
        for i in 0..(max + 1) {
            for j in 0..(max + 1) {
                if w[i][k] && w[k][j] {
                    w[i][j] = true;
                }
            }
        }
    }

    b.iter().enumerate().all(|(i, &b)| w[a[i]][b])
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
