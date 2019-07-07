fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let height: usize = sc.read();
    let width: usize = sc.read();
    let h: usize = sc.read();
    let w: usize = sc.read();
    if height % h == 0 && width % w == 0 {
        println!("No");
        return;
    }
    println!("Yes");
    if width % w != 0 {
        let row = construct(w, width);
        for _ in 0..height {
            for j in 0..width {
                print!("{} ", row[j]);
            }
            println!();
        }
    } else {
        let column = construct(h, height);
        for i in 0..height {
            for _ in 0..width {
                print!("{} ", column[i]);
            }
            println!();
        }
    }
}

fn construct(w: usize, width: usize) -> Vec<i64> {
    let mut sum = vec![0; width + 1];
    for i in 0..width {
        if i >= w {
            sum[i] = sum[i - w] - 1;
        }
    }
    sum[width] = 1;
    let mut cur = width;
    while cur >= w {
        cur -= w;
        sum[cur] = sum[cur + w] + 1;
    }
    let mut a = vec![0; width];
    for i in 0..width {
        a[i] = sum[i + 1] - sum[i];
    }
    a
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
