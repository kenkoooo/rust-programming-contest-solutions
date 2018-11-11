use std::cmp;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    i: usize,
    cost: i64,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut sc = scanner::Scanner::new(stdin.lock());
    let x: usize = sc.read();
    let y: usize = sc.read();
    let z: usize = sc.read();
    let n = x + y + z;
    let mut a: Vec<i64> = vec![];
    let mut b: Vec<i64> = vec![];
    let mut c: Vec<i64> = vec![];
    for _ in 0..n {
        a.push(sc.read());
        b.push(sc.read());
        c.push(sc.read());
    }

    let mut s = (0..n)
        .map(|i| (a[i] - b[i], a[i], b[i], c[i]))
        .collect::<Vec<_>>();
    s.sort();

    let mut heap = BinaryHeap::new();
    let mut b_sum = 0;
    let mut c_sum = 0;
    let mut max_bc = vec![(0, 0); n];
    for (i, &(_, _, b, c)) in s.iter().enumerate() {
        heap.push(State { i: i, cost: b - c });
        b_sum += b;
        if heap.len() == y + 1 {
            let min = heap.pop().unwrap();
            let (_, _, b, c) = s[min.i];
            b_sum -= b;
            c_sum += c;
        }
        max_bc[i] = (b_sum, c_sum);
    }

    let mut heap = BinaryHeap::new();
    let mut a_sum = 0;
    let mut c_sum = 0;
    let mut max_ac = vec![(0, 0); n];
    for (i, &(_, a, _, c)) in s.iter().enumerate().rev() {
        heap.push(State { i: i, cost: a - c });
        a_sum += a;
        if heap.len() == x + 1 {
            let min = heap.pop().unwrap();
            let (_, a, _, c) = s[min.i];
            a_sum -= a;
            c_sum += c;
        }
        max_ac[i] = (a_sum, c_sum);
    }

    let mut ans = 0;
    for i in (y - 1)..n {
        if i + 1 > n - x {
            break;
        }
        let (a, c1) = max_ac[i + 1];
        let (b, c2) = max_bc[i];
        ans = cmp::max(ans, a + b + c1 + c2);
    }
    println!("{}", ans);
}

pub mod scanner {
    use std::fmt::Debug;
    use std::io::Read;
    use std::str::{self, FromStr};

    pub struct Scanner<R: Read> {
        reader: R,
        buf: Vec<u8>,
    }

    impl<R: Read> Scanner<R> {
        pub fn new(reader: R) -> Self {
            Scanner {
                reader: reader,
                buf: Vec::new(),
            }
        }

        pub fn read<T>(&mut self) -> T
        where
            T: FromStr,
            T::Err: Debug,
        {
            self.buf.clear();
            for c in self
                .reader
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n')
                .take_while(|&b| b != b' ' && b != b'\n')
            {
                self.buf.push(c);
            }

            unsafe { str::from_utf8_unchecked(&self.buf) }
                .parse()
                .expect("Parse error.")
        }
    }
}
