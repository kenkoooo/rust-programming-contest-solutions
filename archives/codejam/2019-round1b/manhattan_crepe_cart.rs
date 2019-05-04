fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let t: usize = sc.read();
    for t in 0..t {
        let p: usize = sc.read();
        let q: usize = sc.read();
        let mut sum_x: Vec<i64> = vec![0; q + 2];
        let mut sum_y: Vec<i64> = vec![0; q + 2];
        for _ in 0..p {
            let x: usize = sc.read();
            let y: usize = sc.read();
            let toward: char = sc.read();
            match toward {
                'N' => {
                    sum_y[y + 1] += 1;
                    sum_y[q + 1] -= 1;
                }
                'S' => {
                    sum_y[0] += 1;
                    sum_y[y] -= 1;
                }
                'E' => {
                    sum_x[x + 1] += 1;
                    sum_x[q + 1] -= 1;
                }
                'W' => {
                    sum_x[0] += 1;
                    sum_x[x] -= 1;
                }
                _ => unreachable!(),
            }
        }

        for i in 0..(q + 1) {
            sum_x[i + 1] += sum_x[i];
            sum_y[i + 1] += sum_y[i];
        }
        let (_, x) = (0..(q + 1)).map(|i| (-sum_x[i], i)).min().unwrap();
        let (_, y) = (0..(q + 1)).map(|i| (-sum_y[i], i)).min().unwrap();
        println!("Case #{}: {} {}", t + 1, x, y);
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
