use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let mut map = BTreeMap::new();
    for &a in a.iter() {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut keys = map.keys().map(|key| *key).collect::<Vec<_>>();
    keys.sort();
    let n = keys.len();
    let max = keys[n - 1];
    let max_count = *map.get(&max).unwrap();

    for a in a.into_iter() {
        if max_count > 1 {
            println!("{}", max);
        } else if a == max {
            println!("{}", keys[n - 2]);
        } else {
            println!("{}", max);
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
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
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
