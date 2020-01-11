use std::cmp;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let mut a: Vec<i64> = sc.vec(n);
    let mut total_min = *a.iter().min().unwrap();
    let mut even_min = a
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, &a)| a)
        .min()
        .unwrap();

    let mut total_sum = 0;
    let mut even_sum = 0;
    let mut ans = 0;
    let q: usize = sc.read();
    for _ in 0..q {
        let t: usize = sc.read();
        match t {
            1 => {
                let i = sc.read::<usize>() - 1;
                let count: i64 = sc.read();
                if i % 2 == 0 {
                    let cur = a[i] + even_sum + total_sum;
                    if cur >= count {
                        a[i] -= count;
                        even_min = cmp::min(even_min, a[i]);
                        ans += count;
                    }
                } else {
                    let cur = a[i] + total_sum;
                    if cur >= count {
                        a[i] -= count;
                        total_min = cmp::min(total_min, a[i]);
                        ans += count;
                    }
                }
            }
            2 => {
                let count: i64 = sc.read();
                if even_min + total_sum + even_sum >= count {
                    even_sum -= count;
                    let num = (n + 1) / 2;
                    ans += count * (num as i64);
                }
            }
            3 => {
                let count: i64 = sc.read();
                let min = cmp::min(even_min + total_sum + even_sum, total_min + total_sum);
                if min >= count {
                    total_sum -= count;
                    ans += count * (n as i64);
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
