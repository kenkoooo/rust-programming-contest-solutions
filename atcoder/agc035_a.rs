use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(n);

    if solve(a) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(a: Vec<u64>) -> bool {
    let n = a.len();
    let mut count = BTreeMap::new();
    for &a in a.iter() {
        *count.entry(a).or_insert(0) += 1;
    }
    let mut count = count.into_iter().collect::<Vec<(u64, usize)>>();
    count.sort();
    if n % 3 != 0 {
        return count.len() == 1 && count[0].0 == 0;
    }
    if count.len() == 3 {
        let ((a, ac), (b, bc), (c, cc)) = (count[0], count[1], count[2]);
        ac == bc && ac == cc && a ^ b ^ c == 0
    } else if count.len() == 2 {
        let ((a, ac), (_, bc)) = (count[0], count[1]);
        a == 0 && ac * 2 == bc
    } else if count.len() == 1 {
        count[0].0 == 0
    } else {
        false
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
