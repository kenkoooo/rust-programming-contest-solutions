fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<Vec<char>> = (0..n).map(|_| sc.chars()).collect();

    if a.iter().all(|row| row.iter().all(|&c| c == '.')) {
        println!("-1");
        return;
    }

    let min = (0..n)
        .map(|row| {
            let white = a[row].iter().filter(|&&c| c == '.').count();
            if white == 0 {
                0
            } else if a.iter().any(|r| r[row] == '#') {
                white
            } else {
                white + 1
            }
        })
        .min()
        .unwrap();
    let white_cols = (0..n)
        .filter(|&col| a.iter().any(|row| row[col] == '.'))
        .count();
    println!("{}", min + white_cols);
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
