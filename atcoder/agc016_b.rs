fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let max: usize = a.iter().cloned().max().unwrap();
    let mut count = vec![0; max + 1];
    for a in a.into_iter() {
        count[a] += 1;
    }

    let candidates = count
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count > 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    if candidates.len() > 2 {
        println!("No");
        return;
    }

    if candidates.len() == 1 {
        let count = candidates[0];
        if count <= n / 2 || count == n - 1 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }
    let (c1, c2) = (candidates[0], candidates[1]);
    if count[c2] <= 1 || c1 + 1 != c2 {
        println!("No");
        return;
    }

    let different = count[c1];
    let same = count[c2];
    if c2 < different + 1 || different + same / 2 < c2 {
        println!("No");
        return;
    }

    println!("Yes");
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
