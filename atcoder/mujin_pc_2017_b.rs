use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<Vec<char>> = (0..n).map(|_| sc.chars()).collect::<Vec<_>>();

    if a.iter().all(|row| row.iter().all(|&c| c == '.')) {
        println!("-1");
        return;
    }

    let mut ans = n * n;
    for row_id in 0..n {
        let white = a[row_id].iter().filter(|&&c| c == '.').count();
        let white_columns = (0..n)
            .filter(|&column_id| (0..n).any(|row_id| a[row_id][column_id] == '.'))
            .count();
        if a.iter().any(|row| row[row_id] == '#') {
            ans = cmp::min(ans, white + white_columns);
        } else {
            ans = cmp::min(ans, white + white_columns + 1);
        }
    }
    println!("{}", ans);
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
