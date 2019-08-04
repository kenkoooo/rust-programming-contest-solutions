fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut pairs = vec![];
    if n % 2 == 0 {
        for i in 0..(n / 2) {
            pairs.push(vec![i + 1, n - i]);
        }
    } else {
        for i in 0..(n / 2 + 1) {
            pairs.push(vec![i, n - i]);
        }
    }

    let m = pairs.len();
    let mut edges = vec![];
    for i in 0..m {
        for j in 0..i {
            for &from in pairs[i].iter() {
                for &to in pairs[j].iter() {
                    if from == 0 || to == 0 {
                        continue;
                    }
                    edges.push((from, to));
                }
            }
        }
    }

    println!("{}", edges.len());
    for (a, b) in edges.into_iter() {
        println!("{} {}", a, b);
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
