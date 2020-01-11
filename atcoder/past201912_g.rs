use std::collections::VecDeque;

const INF: i64 = 1e16 as i64;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let mut value = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let v = sc.read::<i64>();
            value[i][j] = v;
            value[j][i] = v;
        }
    }

    let mut q = VecDeque::new();
    let mut dp = vec![vec![-INF; 1 << n]; 4];
    dp[0][0] = 0;
    q.push_back((0, 0));
    while let Some((cur, groups)) = q.pop_front() {
        if groups >= 3 {
            continue;
        }
        for mask in 1..(1 << n) {
            if mask & cur != 0 {
                continue;
            }

            let mut members = vec![];
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    members.push(i);
                }
            }

            let mut sum = 0;
            let m = members.len();
            for i in 0..m {
                for j in (i + 1)..m {
                    sum += value[members[i]][members[j]];
                }
            }

            let next = cur | mask;
            if dp[groups + 1][next] < dp[groups][cur] + sum {
                dp[groups + 1][next] = dp[groups][cur] + sum;
                q.push_back((next, groups + 1));
            }
        }
    }

    let t = (1 << n) - 1;
    let ans = dp.iter().map(|d| d[t]).max().unwrap();
    println!("{}", ans);
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
