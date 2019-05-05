fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n = sc.read();
    let a: Vec<i64> = sc.read_vec(n);
    let abs_max = (0..n)
        .map(|i| (a[i].abs(), i))
        .max()
        .map(|(_, i)| i)
        .unwrap();
    let mut b: Vec<i64> = a.iter().map(|&ai| ai + a[abs_max]).collect();

    let all_positive = b.iter().all(|&x| x >= 0);
    println!("{}", 2 * n - 1);
    for i in 0..n {
        println!("{} {}", abs_max + 1, i + 1);
    }
    if all_positive {
        for i in 1..n {
            println!("{} {}", i, i + 1);
            b[i] += b[i - 1];
        }
    } else {
        for i in (1..n).rev() {
            println!("{} {}", i + 1, i);
            b[i - 1] += b[i];
        }
    }
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
