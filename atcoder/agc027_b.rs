use std::cmp;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let fixed_cost: usize = sc.read();
    let x: Vec<usize> = sc.read_vec(n);
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + x[i];
    }

    let mut ans = 1e18 as usize;
    for count in 1..(n + 1) {
        let mut sub = 0;
        let mut multiply = 3;
        let mut head = n;
        while head > 0 {
            let tmp = if multiply == 3 { 5 } else { multiply };
            if head >= count {
                sub += (sum[head] - sum[head - count]) * tmp;
                head -= count;
            } else {
                sub += (sum[head] - sum[0]) * tmp;
                head = 0;
            }
            multiply += 2;
        }
        sub += count * fixed_cost + n * fixed_cost;
        ans = cmp::min(ans, sub);
    }

    println!("{}", ans);
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
