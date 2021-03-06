fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut a: Vec<usize> = sc.vec::<usize>(n).into_iter().filter(|&a| a < k).collect();
    a.sort();
    if a.is_empty() || is_needed(0, &a, k) {
        sc.write("0\n".to_string());
        return;
    }
    let n = a.len();
    let mut no_need = 0;
    let mut needed = n;
    while needed - no_need > 1 {
        let m = (needed + no_need) / 2;
        if is_needed(m, &a, k) {
            needed = m;
        } else {
            no_need = m;
        }
    }
    sc.write(format!("{}\n", no_need + 1));
}

fn is_needed(i: usize, a: &Vec<usize>, k: usize) -> bool {
    let mut dp = vec![false; k];
    dp[0] = true;
    for (j, &a) in a.iter().enumerate() {
        if j == i {
            continue;
        }
        for l in (0..k).rev() {
            if l + a < k {
                dp[l + a] |= dp[l];
            }
        }
    }
    for t in 0..k {
        if dp[t] && t + a[i] >= k {
            return true;
        }
    }
    false
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write(&mut self, s: String) {
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
