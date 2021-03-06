use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let a: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..m).map(|_| sc.read::<usize>() - 1).collect())
        .collect();

    let mut ans = n;
    let mut pos: Vec<usize> = vec![0; n];
    let mut dead = vec![false; m];
    for _ in 0..m {
        let mut count = vec![0; m];
        for i in 0..n {
            count[a[i][pos[i]]] += 1;
        }
        let (max, max_i) = count
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .max()
            .unwrap();
        ans = cmp::min(ans, max);
        dead[max_i] = true;
        for i in 0..n {
            while pos[i] < m && dead[a[i][pos[i]]] {
                pos[i] += 1;
            }
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
