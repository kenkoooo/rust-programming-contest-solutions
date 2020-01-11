fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut follow = vec![vec![false; n]; n];
    for _ in 0..q {
        let t: usize = sc.read();
        match t {
            1 => {
                let from = sc.read::<usize>() - 1;
                let to = sc.read::<usize>() - 1;
                follow[from][to] = true;
            }
            2 => {
                let user = sc.read::<usize>() - 1;
                for follower in 0..n {
                    if follow[follower][user] {
                        follow[user][follower] = true;
                    }
                }
            }
            3 => {
                let user = sc.read::<usize>() - 1;
                let mut candidates = vec![];
                for x in 0..n {
                    if !follow[user][x] {
                        continue;
                    }
                    for i in 0..n {
                        if follow[x][i] && i != user {
                            candidates.push(i);
                        }
                    }
                }

                for i in candidates.into_iter() {
                    follow[user][i] = true;
                }
            }
            _ => unreachable!(),
        }
    }

    for i in 0..n {
        for j in 0..n {
            sc.write(format!("{}", if follow[i][j] { 'Y' } else { 'N' }));
        }
        sc.write("\n");
    }
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
