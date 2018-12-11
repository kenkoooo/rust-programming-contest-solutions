fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let k: usize = sc.read();
    let n: usize = sc.read();
    if k % 2 == 0 {
        assert_eq!(k % 2, 0);
        print!("{}", k / 2);
        for _ in 1..n {
            print!(" {}", k);
        }
        println!();
    } else {
        let mut ans = vec![(k + 1) / 2; n];
        let elimination = (n - 1 + 1) / 2;
        for _ in 0..elimination {
            let len = ans.len();
            let last = ans[len - 1];
            if last == 1 {
                ans.pop();
            } else {
                ans[len - 1] -= 1;
                while ans.len() < n {
                    ans.push(k);
                }
            }
        }

        for i in 0..ans.len() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", ans[i]);
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
