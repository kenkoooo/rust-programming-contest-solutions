use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut count: Vec<usize> = vec![0; n];
    for _ in 0..n {
        let a = sc.read::<usize>() - 1;
        count[a] += 1;
    }

    count.sort();
    let mut ans = vec![0; n + 1];
    let mut sum = count.iter().sum::<usize>();
    let mut popped = 0;
    for height in (1..(n + 1)).rev() {
        while let Some(c) = count.pop() {
            if c >= height {
                sum -= c;
                popped += 1;
            } else {
                count.push(c);
                break;
            }
        }

        let columns = sum / height + popped;
        ans[columns] = cmp::max(ans[columns], height);
    }
    for k in (0..n).rev() {
        ans[k] = cmp::max(ans[k], ans[k + 1]);
    }

    for k in 1..(n + 1) {
        println!("{}", ans[k]);
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
