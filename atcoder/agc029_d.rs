use std::cmp;
use std::collections::BTreeSet;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let h: usize = sc.read();
    let w: usize = sc.read();
    let n: usize = sc.read();

    let mut x_set = vec![BTreeSet::new(); w + 1];
    let mut y_set = vec![BTreeSet::new(); h + 1];
    for _ in 0..n {
        let x: usize = sc.read();
        let y: usize = sc.read();
        x_set[y].insert(x);
        y_set[x].insert(y);
    }

    for i in 1..(h + 1) {
        y_set[i].insert(w + 1);
    }
    for j in 1..(w + 1) {
        x_set[j].insert(h + 1);
    }

    let mut ans = std::usize::MAX;
    let mut y = 1;
    for x in 1.. {
        if x_set[y].contains(&(x + 1)) {
            ans = cmp::min(ans, x);
            break;
        }
        let x = x + 1;

        if let Some(&wx) = x_set[y].range((x + 1)..).next() {
            ans = cmp::min(ans, wx - 1);
        }

        if !y_set[x].contains(&(y + 1)) {
            y += 1;
        }
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
