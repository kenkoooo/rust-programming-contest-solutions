fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();

    let board: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut blue = vec![vec!['.'; w]; h];
    let mut red = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if j == 0 {
                red[i][j] = '#';
            } else if j == w - 1 {
                blue[i][j] = '#';
            } else if i % 2 == 0 {
                red[i][j] = '#';
            } else {
                blue[i][j] = '#';
            }

            if board[i][j] == '#' {
                red[i][j] = '#';
                blue[i][j] = '#';
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", red[i][j]);
        }
        println!();
    }
    println!();
    for i in 0..h {
        for j in 0..w {
            print!("{}", blue[i][j]);
        }
        println!();
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
