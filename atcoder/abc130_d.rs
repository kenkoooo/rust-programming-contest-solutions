fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let k: u64 = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let mut tail = 0;
    let mut sum = 0;
    let mut ans = 0;
    for head in 0..n {
        while tail <= head {
            sum += a[tail];
            tail += 1;
        }
        while tail < n && sum + a[tail] < k {
            sum += a[tail];
            tail += 1;
        }

        // [head, tail)
        //        let z: u64 = a[head..tail].iter().sum();
        //        eprintln!("head={} tail={} {:?} {}", head, tail, &a[head..tail], z);
        ans += tail - head;
        if tail - head == 1 && a[head] >= k {
            ans -= 1;
        }

        sum -= a[head];
    }
    println!("{}", n * (n + 1) / 2 - ans);
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
