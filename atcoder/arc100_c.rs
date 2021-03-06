use std::cmp;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(1 << n);
    let mut top2 = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| ((a, i), (0, 0)))
        .collect::<Vec<_>>();
    let mut masks = (0..(1usize << n))
        .map(|mask| (mask.count_ones(), mask))
        .collect::<Vec<_>>();
    masks.sort();
    for (_, mask) in masks.into_iter() {
        for i in 0..n {
            if mask & (1 << i) != 0 {
                continue;
            }
            let next_mask = mask | (1 << i);
            top2[next_mask] = add(top2[next_mask], top2[mask].0);
            top2[next_mask] = add(top2[next_mask], top2[mask].1);
        }
    }

    let mut max = 0;
    for i in 1..(1 << n) {
        let ((a, _), (b, _)) = top2[i];
        max = cmp::max(max, a + b);
        sc.write(format!("{}\n", max));
    }
}

fn add<T: PartialOrd>((a, b): (T, T), v: T) -> (T, T) {
    if a == v || b == v {
        (a, b)
    } else if v > a {
        (v, a)
    } else if v > b {
        (a, v)
    } else {
        (a, b)
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
