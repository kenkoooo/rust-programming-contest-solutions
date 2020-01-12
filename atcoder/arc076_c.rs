use std::collections::VecDeque;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let r: i64 = sc.read();
    let c: i64 = sc.read();
    let n: usize = sc.read();
    let mut edges = vec![];
    for _ in 0..n {
        let x1: i64 = sc.read();
        let y1: i64 = sc.read();
        let x2: i64 = sc.read();
        let y2: i64 = sc.read();
        let e1 = x1 == 0 || x1 == r || y1 == 0 || y1 == c;
        let e2 = x2 == 0 || x2 == r || y2 == 0 || y2 == c;
        if e1 && e2 {
            edges.push((x1, y1, x2, y2));
        }
    }

    let mut points = vec![];
    for (i, (x1, y1, x2, y2)) in edges.into_iter().enumerate() {
        for (x, y) in vec![(x1, y1), (x2, y2)].into_iter() {
            if y == 0 {
                points.push((0, x, i));
            } else if x == r {
                points.push((1, y, i));
            } else if y == c {
                points.push((2, -x, i));
            } else {
                points.push((3, -y, i));
            }
        }
    }

    points.sort();
    let mut stack = VecDeque::new();
    for (_, _, i) in points.into_iter() {
        if let Some(top) = stack.pop_front() {
            if top == i {
                continue;
            }
            stack.push_front(top);
        }
        stack.push_front(i);
    }

    if stack.is_empty() {
        println!("YES");
    } else {
        println!("NO");
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
