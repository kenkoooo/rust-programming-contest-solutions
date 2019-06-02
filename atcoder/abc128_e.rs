use std::collections::BTreeSet;

#[derive(Ord, Eq, PartialEq, PartialOrd)]
enum QueryType {
    Start = 2,
    Stop = 1,
    Query = 3,
}

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let q: usize = sc.read();
    let mut events = vec![];
    for _ in 0..n {
        let s: i64 = sc.read();
        let t: i64 = sc.read();
        let x: i64 = sc.read();
        events.push((s - x, QueryType::Start, x));
        events.push((t - x, QueryType::Stop, x));
    }
    for _ in 0..q {
        let d: i64 = sc.read();
        events.push((d, QueryType::Query, 0));
    }
    events.sort();

    let mut stopping_heap = BTreeSet::new();
    for (_, t, x) in events.into_iter() {
        match t {
            QueryType::Start => {
                stopping_heap.insert(x);
            }
            QueryType::Stop => {
                stopping_heap.remove(&x);
            }
            QueryType::Query => match stopping_heap.iter().next() {
                Some(x) => {
                    println!("{}", x);
                }
                None => {
                    println!("-1");
                }
            },
        }
    }
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
