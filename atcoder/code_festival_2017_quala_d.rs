const LENGTH: usize = 2000;
const S: usize = 500;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let d: usize = sc.read();

    let mut big_table = vec![vec!['?'; LENGTH]; LENGTH];
    let colors = vec![vec!['R', 'G'], vec!['B', 'Y']];
    for i in 0..LENGTH {
        for j in 0..LENGTH {
            let ii = (i / d) % 2;
            let jj = (j / d) % 2;
            big_table[i][j] = colors[ii][jj];
        }
    }

    for i in 0..h {
        for j in 0..w {
            let p = S + i + j;
            let q = S + i - j;
            print!("{}", big_table[p][q]);
        }
        println!();
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
