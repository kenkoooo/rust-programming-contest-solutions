fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let mut ans = vec![' '; 2 * n];

    print_n(1, n);
    let first = sc.chars()[0];
    print_n(n + 1, n);
    let last = sc.chars()[0];

    let mut prefix = 1;
    let mut suffix = n + 1;
    while suffix - prefix > 1 {
        let m = (suffix + prefix) / 2;
        print_n(m, n);
        let c = sc.chars()[0];
        if c == first {
            prefix = m;
        } else {
            suffix = m;
        }
    }
    ans[prefix - 1] = first;
    ans[suffix + n - 1 - 1] = last;
    // [suffix, ... , suffix+n-2].len() = n-1
    for i in 1..prefix {
        print_1(i, suffix, n);
        let c = sc.chars()[0];
        ans[i - 1] = c;
    }
    for i in (suffix + n)..(2 * n + 1) {
        print_1(i, suffix, n);
        let c = sc.chars()[0];
        ans[i - 1] = c;
    }

    let mut primer = vec![];
    for i in 0..(2 * n) {
        if ans[i] == first && primer.len() < (n - 1) / 2 {
            primer.push(i + 1);
        }
    }
    for i in 0..(2 * n) {
        if ans[i] == last && primer.len() < (n - 1) {
            primer.push(i + 1);
        }
    }

    for i in 0..(2 * n) {
        if ans[i] != last && ans[i] != first {
            print_v(i + 1, &primer);
            ans[i] = sc.chars()[0];
        }
    }

    print!("! ");
    for i in 0..(2 * n) {
        print!("{}", ans[i]);
    }
    println!();
}

fn print_v(i: usize, v: &Vec<usize>) {
    print!("? {}", i);
    for &i in v.iter() {
        print!(" {}", i);
    }
    println!()
}

fn print_1(i: usize, head: usize, n: usize) {
    print!("? {}", i);
    for i in 0..(n - 1) {
        print!(" {}", i + head);
    }
    println!();
}

fn print_n(head: usize, n: usize) {
    print!("?");
    for i in 0..n {
        print!(" {}", head + i);
    }
    println!();
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
