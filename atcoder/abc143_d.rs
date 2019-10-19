fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let l: Vec<usize> = sc.vec(n);

    let mut two = vec![0; 10000];
    for i in 0..n {
        for j in (i + 1)..n {
            let x = l[i] + l[j];
            two[x] += 1;
        }
    }

    let mut ans = n * (n - 1) * (n - 2) / 6;
    for sum_length in 0..two.len() {
        let two_count = two[sum_length];
        if two_count == 0 {
            continue;
        }
        for &single in l.iter() {
            if sum_length <= single {
                ans -= two_count;
            }
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
