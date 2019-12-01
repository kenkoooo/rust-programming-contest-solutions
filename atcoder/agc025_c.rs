use std::cmp;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let lr = (0..n)
        .map(|_| (sc.read::<i64>(), sc.read::<i64>()))
        .collect::<Vec<_>>();

    let mut left = vec![];
    let mut right = vec![];
    for i in 0..n {
        let (l, r) = lr[i];
        left.push(2 * l);
        right.push(-2 * r);
    }
    left.sort();
    left.reverse();
    right.sort();
    right.reverse();

    let mut max = 0;
    let mut sum = 0;
    for i in 0..n {
        max = cmp::max(max, sum);
        max = cmp::max(max, sum + left[i]);
        max = cmp::max(max, sum + right[i]);
        sum += left[i] + right[i];
    }

    println!("{}", max);
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
