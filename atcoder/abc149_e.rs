fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut a: Vec<u64> = sc.vec(n);
    a.sort();

    let mut suffix_sum = vec![0; n + 1];
    for i in 0..n {
        suffix_sum[i + 1] = suffix_sum[i] + a[n - 1 - i];
    }

    let mut ng = 1e15 as u64;
    let mut ok = 0;
    while ng - ok > 1 {
        let x = (ok + ng) >> 1;
        let (count, _) = calc(x, &a, &suffix_sum);

        if count >= m {
            ok = x;
        } else {
            ng = x;
        }
    }

    let (count1, sum1) = calc(ok, &a, &suffix_sum);
    if count1 == m {
        println!("{}", sum1);
        return;
    }

    let (count2, sum2) = calc(ok + 1, &a, &suffix_sum);
    assert!(count2 < m);
    let remain = m - count2;
    let ans = sum2 + ok * remain as u64;

    println!("{}", ans);
}

fn calc(x: u64, a: &Vec<u64>, suffix_sum: &Vec<u64>) -> (usize, u64) {
    let n = a.len();
    let mut sum = 0;
    let mut cur_row = n;
    let mut cur_col = 0;
    let mut count = 0;
    while cur_col < n {
        if cur_row > 0 && a[cur_col] + a[cur_row - 1] >= x {
            cur_row -= 1;
        } else {
            let c = n - cur_row;
            sum += suffix_sum[c] + a[cur_col] * c as u64;
            count += c;
            cur_col += 1;
        }
    }
    (count, sum)
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
