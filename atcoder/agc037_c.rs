use std::collections::BinaryHeap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let mut b: Vec<u64> = sc.vec(n);
    if (0..n).any(|i| a[i] > b[i]) {
        println!("-1");
        return;
    }

    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        push(&mut heap, &a, &b, i);
    }

    while let Some((count, neighbors, i)) = heap.pop() {
        ans += count;
        b[i] -= count * neighbors;
        assert!(b[i] >= a[i]);
        push(&mut heap, &a, &b, (i + 1) % n);
        push(&mut heap, &a, &b, (n + i - 1) % n);
    }

    if a != b {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn push(heap: &mut BinaryHeap<(u64, u64, usize)>, a: &Vec<u64>, b: &Vec<u64>, i: usize) {
    let n = b.len();
    let prev = b[(n + i - 1) % n];
    let next = b[(i + 1) % n];
    let neighbors = prev + next;
    let db = b[i] - a[i];
    let count = db / neighbors;
    if count > 0 {
        heap.push((count, neighbors, i));
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
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
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
