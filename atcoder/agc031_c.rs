fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let a: u64 = sc.read();
    let b: u64 = sc.read();
    let ans = construct(a, b, 1 << n);
    if ans.is_empty() {
        println!("NO");
    } else {
        sc.write("YES\n");
        for (i, ans) in ans.into_iter().enumerate() {
            if i > 0 {
                sc.write(" ");
            }
            sc.write(format!("{}", ans));
        }
        sc.write("\n");
    }
}

fn construct(first: u64, last: u64, n: usize) -> Vec<u64> {
    let diff = first ^ last;
    if diff.count_ones() % 2 == 0 {
        return Vec::new();
    }
    if n == 2 {
        return vec![first, last];
    }
    let diff_pos = (0..).find(|&i| diff & (1 << i) != 0).unwrap();

    let prefix_add = (first >> diff_pos) & 1;
    let suffix_add = (last >> diff_pos) & 1;
    assert_ne!(prefix_add, suffix_add);

    let prefix_first = remove(first, diff_pos);
    let prefix_last = prefix_first ^ 1;
    let suffix_first = prefix_first ^ 1;
    let suffix_last = remove(last, diff_pos);

    let prefix = construct(prefix_first, prefix_last, n / 2);
    let suffix = construct(suffix_first, suffix_last, n / 2);
    if prefix.is_empty() || suffix.is_empty() {
        return Vec::new();
    }

    let mut v = vec![];
    for t in prefix.into_iter() {
        v.push(insert(t, diff_pos, prefix_add));
    }
    for t in suffix.into_iter() {
        v.push(insert(t, diff_pos, suffix_add));
    }

    v
}

fn remove(v: u64, pos: u64) -> u64 {
    let suffix = v & ((1 << pos) - 1);
    let prefix = (v >> (pos + 1)) << pos;
    assert_eq!(prefix & suffix, 0);
    prefix + suffix
}

fn insert(v: u64, pos: u64, add: u64) -> u64 {
    let suffix = v & ((1 << pos) - 1);
    let prefix = (((v >> pos) << 1) + add) << pos;
    assert_eq!(prefix & suffix, 0);
    prefix + suffix
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
