use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let x: usize = sc.read();
    let y: usize = sc.read();
    let z: usize = sc.read();
    let n = x + y + z;
    let mut abc = vec![];
    for i in 0..n {
        let a: i64 = sc.read();
        let b: i64 = sc.read();
        let c: i64 = sc.read();
        abc.push((a, b, c));
    }
    abc.sort_by_key(|&(a, b, _)| b - a);

    let mut cur_x = 0;
    let mut prefix = vec![0; z + 1];
    let mut q = BinaryHeap::new();

    for i in 0..x {
        let (a, _, c) = abc[i];
        cur_x += a;
        q.push((c - a, a, c));
    }

    prefix[0] = cur_x;
    let mut cur_z = 0;
    for i in 0..z {
        let (a, _, c) = abc[i + x];
        cur_x += a;
        q.push((c - a, a, c));

        let (_, a, c) = q.pop().unwrap();
        cur_x -= a;
        cur_z += c;
        prefix[i + 1] = cur_x + cur_z;
    }

    let mut q = BinaryHeap::new();
    let mut suffix = vec![0; z + 1];
    let mut cur_y = 0;
    for i in 0..y {
        let (_, b, c) = abc[n - 1 - i];
        cur_y += b;
        q.push((c - b, b, c));
    }
    suffix[z] = cur_y;
    let mut cur_z = 0;
    for i in 0..z {
        let (_, b, c) = abc[n - 1 - y - i];
        cur_y += b;
        q.push((c - b, b, c));

        let (_, b, c) = q.pop().unwrap();
        cur_y -= b;
        cur_z += c;
        suffix[z - 1 - i] = cur_y + cur_z;
    }

    let mut ans = 0;
    for i in 0..(z + 1) {
        ans = cmp::max(ans, prefix[i] + suffix[i]);
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
