use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let a: Vec<usize> = sc.vec(n);

    if (1..n).all(|i| a[i] > a[i - 1]) {
        println!("1");
        return;
    }

    let mut ok = n;
    let mut ng = 0;
    while ok - ng > 1 {
        let x = (ok + ng) / 2;
        if solve(&a, x) {
            ok = x;
        } else {
            ng = x;
        }
    }

    println!("{}", ok + 1);
}

fn solve(a: &Vec<usize>, limit: usize) -> bool {
    let mut state = State {
        map: BTreeMap::new(),
        limit: limit,
    };

    let n = a.len();
    let mut start = 0;
    for i in 1..n {
        if a[i] <= a[i - 1] {
            state.increment(a[i]);
            start = i + 1;
            break;
        }
    }

    for i in start..n {
        if a[i] <= a[i - 1] {
            state.resize(a[i]);
            if !state.increment(a[i]) {
                return false;
            }
        }
    }
    true
}

struct State {
    map: BTreeMap<usize, usize>,
    limit: usize,
}

impl State {
    fn increment(&mut self, i: usize) -> bool {
        if let Some(&cur) = self.map.get(&i) {
            if cur == self.limit {
                self.map.remove(&i);
                if i == 1 {
                    false
                } else {
                    self.increment(i - 1)
                }
            } else {
                *self.map.get_mut(&i).unwrap() += 1;
                true
            }
        } else {
            self.map.insert(i, 1);
            true
        }
    }
    fn resize(&mut self, size: usize) {
        while let Some(&key) = self.map.keys().next_back() {
            if key > size {
                self.map.remove(&key);
            } else {
                break;
            }
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
