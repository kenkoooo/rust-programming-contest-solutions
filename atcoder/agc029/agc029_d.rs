use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let n: usize = sc.read();

    let mut map = BTreeMap::new();
    for _ in 0..n {
        let x: usize = sc.read();
        let y: usize = sc.read();
        map.entry(x).or_insert(BTreeSet::new()).insert(y);
    }

    let mut y = 1;
    for x in 1..(h + 1) {
        if let Some(set) = map.get(&(x + 1)) {
            if set.range(..(y + 1)).next().is_some() {
                println!("{}", x);
                return;
            }
            if !set.contains(&(y + 1)) {
                y += 1;
            }
        } else {
            y += 1;
        }
    }
    println!("{}", h);
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
