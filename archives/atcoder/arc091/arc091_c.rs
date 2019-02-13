use std::cmp;
fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { stdin: sc.lock() };
    let n: usize = sc.read();
    let a: usize = sc.read();
    let b: usize = sc.read();

    let segments = (n + b - 1) / b;
    if a + b - 1 > n || (n + b - 1) / b > a {
        println!("-1");
        return;
    }

    let mut ans = (1..(n + 1)).collect::<Vec<usize>>();
    for i in 0..segments {
        let from = i * b;
        let to = cmp::min(from + b, n);
        ans[from..to].reverse();
    }

    let mut a = a - segments;
    for i in (0..segments).rev() {
        if a == 0 {
            break;
        }
        let length = cmp::min(a + 1, b);
        let from = i * b;
        let to = cmp::min(from + length, n);
        ans[from..to].reverse();
        a -= (to - from) - 1;
    }

    for (i, a) in ans.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a);
    }

    println!();
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
