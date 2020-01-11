use std::cmp::max;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let k: usize = sc.read();
    let r: usize = sc.read();
    let s: usize = sc.read();
    let p: usize = sc.read();

    let t = sc.chars();

    let mut ts = vec![vec![]; k];
    for i in 0..n {
        ts[i % k].push(t[i]);
    }

    let mut ans = 0;
    for t in ts.into_iter() {
        let n = t.len();
        let mut dp = [0; 3];
        for i in 0..n {
            let mut next = [0; 3];
            for next_hand in 0..3 {
                // r, s, p
                for prev_hand in 0..3 {
                    if next_hand == prev_hand {
                        continue;
                    }

                    let get = match t[i] {
                        'r' if next_hand == 2 => p,
                        's' if next_hand == 0 => r,
                        'p' if next_hand == 1 => s,
                        _ => 0,
                    };

                    next[next_hand] = max(next[next_hand], dp[prev_hand] + get);
                }
            }
            dp = next;
        }

        ans += max(dp[0], max(dp[1], dp[2]));
    }

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
