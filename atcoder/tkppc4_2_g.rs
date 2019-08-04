fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
}
// a1 a2 a3 a4 a5
// a1 = x
// a2 = y + 4x
// a3 = z + 4y + 9z
// a4 = w + 4z + 9y + 16x
// a5 = v + 4w + 9z + 16y + 25x

// b1 = a2 - a1 = y + 3x
// b2 = z + 3y + 5x
// b3 = w + 3z + 5y + 7x
// b4 = v + 3w + 5z + 7y + 9x

// c1 = z + 2y + 2x
// c2 = w + 2z + 2y + 2x

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
