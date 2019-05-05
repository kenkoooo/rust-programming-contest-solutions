use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let r: f64 = sc.read();
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut lines = vec![0.0; n - 1];
    let dr = 2.0 * r / (n as f64);
    for i in 1..n {
        let dr = r - dr * (i as f64);
        let dh = (r * r - dr * dr).sqrt();
        lines[i - 1] = dh * 2.0;
    }
    let mut used = vec![false; n - 1];
    let mut ans = 0.0;
    for _ in 0..n {
        let mut head = n;
        for (i, &line) in lines.iter().enumerate() {
            if (head == n || lines[head] < line) && !used[i] {
                head = i;
            }
        }

        if head == n {
            break;
        }
        let mut tail = n;
        for (i, &line) in lines.iter().enumerate() {
            if (tail == n || lines[tail] < line)
                && cmp::min(i, head) + m <= cmp::max(i, head)
                && !used[i]
            {
                tail = i;
            }
        }

        if tail == n {
            used[head] = true;
            ans += lines[head];
        } else {
            used[head] = true;
            used[tail] = true;
            assert!(lines[head] >= lines[tail]);
            ans += lines[head];
        }
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
