fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let b: Vec<u64> = sc.vec(n);

    if solve(a, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(a: Vec<u64>, b: Vec<u64>) -> bool {
    let n = a.len();
    let mut ba: Vec<_> = (0..n).map(|i| (b[i], a[i])).collect();
    ba.sort();

    let mut a: Vec<_> = ba.iter().enumerate().map(|(i, &(_, a))| (a, i)).collect();
    a.sort();

    for i in 0..n {
        if a[i].0 > ba[i].0 {
            return false;
        }
    }

    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[cur] {
        vis[cur] = true;
        cur = a[cur].1;
    }

    if vis.contains(&false) {
        return true;
    }

    for i in 1..n {
        if a[i].0 <= ba[i - 1].0 {
            return true;
        }
    }

    false
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
