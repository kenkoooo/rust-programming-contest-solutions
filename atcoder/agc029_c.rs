use std::collections::BTreeMap;

fn solve(a: &Vec<usize>, color: usize) -> bool {
    let n = a.len();
    let mut big = Big {
        map: BTreeMap::new(),
        color: color,
    };
    let mut start = 0;
    for i in 1..n {
        if a[i] <= a[i - 1] {
            big.increment(a[i]);
            start = i + 1;
            break;
        }
    }

    for i in start..n {
        if a[i] <= a[i - 1] {
            big.resize(a[i]);
            if !big.increment(a[i]) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);

    if (1..n).all(|i| a[i] > a[i - 1]) {
        println!("1");
        return;
    }

    let mut ng = 0;
    let mut ok = n;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if solve(&a, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok + 1);
}

struct Big {
    map: BTreeMap<usize, usize>,
    color: usize,
}

impl Big {
    fn increment(&mut self, index: usize) -> bool {
        if let Some(&v) = self.map.get(&index) {
            if v == self.color {
                self.map.remove(&index);
                if index == 1 {
                    false
                } else {
                    self.increment(index - 1)
                }
            } else {
                *self.map.get_mut(&index).unwrap() += 1;
                true
            }
        } else {
            self.map.insert(index, 1);
            true
        }
    }

    fn resize(&mut self, s: usize) {
        while let Some(&key) = self.map.keys().next_back() {
            if key > s {
                self.map.remove(&key);
            } else {
                break;
            }
        }
    }
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
