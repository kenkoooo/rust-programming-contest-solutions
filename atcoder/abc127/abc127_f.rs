use std::collections::BinaryHeap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let q: usize = sc.read();
    let mut center = 0;
    let mut lower = BinaryHeap::new();
    let mut upper = BinaryHeap::new();
    let mut lower_sum = 0;
    let mut upper_sum = 0;

    let mut b: i64 = 0;
    for _ in 0..q {
        let t: usize = sc.read();
        if t == 1 {
            let a: i64 = sc.read();

            if lower.is_empty() {
                lower.push(a);
            } else {
                if a < center {
                    lower.push(a);
                    lower_sum += center - a;
                } else {
                    upper.push(-a);
                    upper_sum += a - center;
                }
            }
            while lower.len() < upper.len() || lower.len() > upper.len() + 1 {
                if lower.len() < upper.len() {
                    let upper_min = -upper.top();
                    let diff = upper_min - center;

                    lower_sum += lower.len() as i64 * diff;
                    upper_sum -= diff * upper.len() as i64;
                    upper.pop().unwrap();
                    lower.push(upper_min);
                } else {
                    let lower_max = lower.pop().unwrap();
                    let diff = center - lower.top();
                    upper.push(-lower_max);

                    lower_sum -= diff * lower.len() as i64;
                    upper_sum += upper.len() as i64 * diff;
                }
                center = lower.top();
            }

            center = lower.top();

            b += sc.read::<i64>();
        } else {
            println!("{} {}", center, b + lower_sum + upper_sum);
        }
    }
}

trait PeekTop<T> {
    fn top(&mut self) -> T;
}

impl<T> PeekTop<T> for BinaryHeap<T>
where
    T: Copy + Ord,
{
    fn top(&mut self) -> T {
        let top = self.pop().unwrap();
        self.push(top);
        top
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
