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
    } else {
        let mut ans = vec![(k + 1) / 2; n];
        let mut tail = ans.len() - 1;
        for _ in 0..(n / 2) {
            ans[tail] -= 1;
            if ans[tail] == 0 {
                tail -= 1;
            } else {
                for i in (tail + 1)..n {
                    ans[i] = k;
                }
                tail = ans.len() - 1;
            }
        }
        for i in 0..n {
            if ans[i] == 0 {
                break;
            }
            if i > 0 {
                print!(" ");
            }
            print!("{}", ans[i]);
        }
        println!();
    }
}

/// 1 ...
/// 2 ...
/// 3
/// 3 1 ...
/// 3 2 ...
/// 3 3 ...
/// 3 4 ...
/// 3 5 ...
/// 4 ...
/// 5 ...
///
/// 3 3 3 3 3 3 3 3 3 3 3 3 0 0
/// 3 3 3 3 3 3 3 3 3 3 3 3 1 0
/// 3 3 3 3 3 3 3 3 3 3 3 3 1 1
/// 3 3 3 3 3 3 3 3 3 3 3 3 2 2
/// 3 3 3 3 3 3 3 3 3 3 3 3 2 3
/// 3 3 3 3 3 3 3 3 3 3 3 3 2 4
/// 3 3 3 3 3 3 3 3 3 3 3 3 2 5
/// 3 3 3 3 3 3 3 3 3 3 3 3 3
/// 3 3 3 3 3 3 3 3 3 3 3 3 3 1
/// 3 3 3 3 3 3 3 3 3 3 3 3 3 2
/// 3 3 3 3 3 3 3 3 3 3 3 3 3 3
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
