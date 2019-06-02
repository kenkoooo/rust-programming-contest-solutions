use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let s: Vec<i64> = sc.vec(n);

    let mut ans = 0;
    for i in 1..n {
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut left_cur = 0;
        let mut right_cur = n - 1;
        while left_cur + i < n {
            left_cur += i;
            right_cur -= i;

            if (n - 1) % i == 0 && left_cur >= right_cur {
                break;
            }

            if right_cur <= i {
                break;
            }

            left_sum += s[left_cur];
            right_sum += s[right_cur];
            ans = cmp::max(ans, left_sum + right_sum);
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
