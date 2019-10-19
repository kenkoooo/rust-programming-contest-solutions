use std::collections::BinaryHeap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let mut b: Vec<u64> = sc.vec(n);
    if (0..n).any(|i| b[i] < a[i]) {
        println!("-1");
        return;
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        try_push(&mut heap, &a, &b, i);
    }

    let mut ans = 0;
    while let Some((_, i)) = heap.pop() {
        let prev = b[(i + n - 1) % n];
        let next = b[(i + 1) % n];
        let diff = b[i] - a[i];
        let count = diff / (prev + next);
        ans += count;
        b[i] -= count * (prev + next);
        try_push(&mut heap, &a, &b, (i + 1) % n);
        try_push(&mut heap, &a, &b, (i + n - 1) % n);
    }
    if a == b {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn try_push(heap: &mut BinaryHeap<(u64, usize)>, a: &[u64], b: &[u64], i: usize) {
    let n = a.len();
    let prev = b[(i + n - 1) % n];
    let next = b[(i + 1) % n];
    let diff = b[i] - a[i];
    let count = diff / (prev + next);
    if count > 0 {
        heap.push((count, i));
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
