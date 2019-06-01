fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut switches = vec![vec![]; m];
    for i in 0..m {
        let k: usize = sc.read();
        for _ in 0..k {
            let s: usize = sc.read::<usize>() - 1;
            switches[i].push(s);
        }
    }
    let p: Vec<usize> = sc.vec(m);

    let mut ans = 0;
    for mask in 0..(1 << n) {
        let mut ok = true;
        for (i, switch) in switches.iter().enumerate() {
            let mut count = 0;
            for &i in switch.iter() {
                if mask & (1 << i) != 0 {
                    count += 1;
                }
            }

            if count % 2 != p[i] {
                ok = false;
                break;
            }
        }

        if ok {
            ans += 1;
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
