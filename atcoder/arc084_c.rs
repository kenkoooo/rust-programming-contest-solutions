fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let k: usize = sc.read();
    let n: usize = sc.read();

    if k % 2 == 0 {
        print!("{}", k / 2);
        for _ in 1..n {
            print!(" {}", k);
        }
        println!();
        return;
    }

    let x = (k + 1) / 2;
    let mut cur = vec![x; n];
    let mut ptr = n - 1;
    for _ in 0..(n / 2) {
        cur[ptr] -= 1;
        if cur[ptr] == 0 {
            ptr -= 1;
        } else {
            while ptr + 1 < n {
                ptr += 1;
                cur[ptr] = k;
            }
        }
    }

    print!("{}", cur[0]);
    for i in 1..n {
        if cur[i] == 0 {
            break;
        }
        print!(" {}", cur[i]);
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
