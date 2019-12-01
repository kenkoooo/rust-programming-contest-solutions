fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let a: Vec<_> = sc.vec(n);

    if solve(&a, 0) {
        println!("1");
        return;
    }
    let mut ok = 1e15 as u64;
    let mut ng = 0;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        if solve(&a, m) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok + 1);
}

fn solve(a: &Vec<usize>, max: u64) -> bool {
    let mut state = State {
        v: vec![],
        length: 0,
        max: max,
    };
    let mut prev = 0;
    for &a in a.iter() {
        state.resize(a);
        if prev >= a {
            if !state.increment() {
                return false;
            }
        }
        prev = a;
    }
    true
}

#[derive(Debug)]
struct State {
    v: Vec<(u64, usize)>,
    length: usize,
    max: u64,
}

impl State {
    fn resize(&mut self, required_length: usize) {
        if self.length <= required_length {
            let add = required_length - self.length;
            if add > 0 {
                self.v.push((0, add));
                self.length += add;
            }
        } else {
            while let Some((c, seg_length)) = self.v.pop() {
                self.length -= seg_length;
                if self.length < required_length {
                    let add = required_length - self.length;
                    self.v.push((c, add));
                    self.length += add;
                    break;
                }
            }
        }
        //        eprintln!("{:?} l={}", self, required_length);
        assert_eq!(self.length, required_length);
    }

    fn increment(&mut self) -> bool {
        let mut tail = 0;
        while let Some((c, seg_length)) = self.v.pop() {
            if c == self.max {
                tail += seg_length;
            } else {
                if seg_length > 1 {
                    self.v.push((c, seg_length - 1));
                }
                self.v.push((c + 1, 1));
                if tail > 0 {
                    self.v.push((0, tail));
                }
                return true;
            }
        }
        false
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
