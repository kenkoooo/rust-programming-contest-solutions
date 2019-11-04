const MOD: usize = 998244353;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let s = sc.chars();

    let mut rgb = vec![vec![]; 3];
    for (i, &c) in s.iter().enumerate() {
        let c = match c {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        };
        rgb[c].push(i);
    }

    let mut min = vec![];
    let mut center = vec![];
    let mut max = vec![];
    for i in 0..n {
        let mut x = [rgb[0][i], rgb[1][i], rgb[2][i]];
        x.sort();

        min.push(x[0]);
        center.push(x[1]);
        max.push(x[2]);
    }

    let mut head = 0;
    let mut from_min = 1;
    for i in 0..n {
        let c = center[i];
        while head < n && min[head] < c {
            head += 1;
        }
        from_min = (from_min * (head - i)) % MOD;
    }

    let mut head = 0;;
    let mut to_min = 1;
    for i in (0..n).rev() {
        let c = center[i];
        while head < n && max[n - 1 - head] > c {
            head += 1;
        }
        to_min = (to_min * (head - (n - 1 - i))) % MOD;
    }

    let mut ans = (from_min * to_min) % MOD;
    for i in 0..n {
        ans = (ans * (i + 1)) % MOD;
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
