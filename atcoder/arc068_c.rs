fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut s = vec![];
    for _ in 0..n {
        let l: usize = sc.read();
        let r: usize = sc.read();
        let w = r - l + 1;
        s.push((w, l, r));
    }

    s.sort();
    let mut seg =
        range_add_segment_tree::RangeAddSegmentTree::<i64, _>::new(m + 1, 0, |a: i64, _: i64| a, 0);
    for i in 0..(m + 1) {
        seg.update(i, 0);
    }
    let mut head = 0;
    for d in 1..(m + 1) {
        while head < n && s[head].0 < d {
            let (_, l, r) = s[head];
            seg.add(l, r + 1, 1);
            head += 1;
        }
        let mut ans = (n - head) as i64;
        let mut pos = 0;
        while pos <= m {
            ans += seg.get(pos, pos + 1);
            pos += d;
        }

        println!("{}", ans);
    }
}

pub mod range_add_segment_tree {
    use std::ops::Add;

    pub struct RangeAddSegmentTree<T, F> {
        seg: Vec<T>,
        seg_add: Vec<T>,
        num: usize,
        f: F,
        init: T,
    }

    impl<T, F> RangeAddSegmentTree<T, F>
    where
        T: PartialOrd + Add<Output = T> + Copy,
        F: FnOnce(T, T) -> T + Clone,
    {
        pub fn new(n: usize, init: T, f: F, zero: T) -> Self {
            let num = n.next_power_of_two();
            RangeAddSegmentTree {
                seg: vec![init; num * 2],
                seg_add: vec![zero; num * 2],
                num: num,
                init: init,
                f: f,
            }
        }

        /// add to [a, b)
        pub fn add(&mut self, a: usize, b: usize, value: T) {
            self.add_to_range(a, b, value, 0, 0, self.num);
        }

        fn add_to_range(
            &mut self,
            a: usize,
            b: usize,
            value: T,
            mut k: usize,
            left: usize,
            right: usize,
        ) {
            if b <= left || right <= a {
                return;
            }
            if a <= left && right <= b {
                self.seg_add[k] = self.seg_add[k] + value;
                while k > 0 {
                    k = (k - 1) / 2;
                    self.seg[k] = (self.f.clone())(
                        self.seg[k * 2 + 1] + self.seg_add[k * 2 + 1],
                        self.seg[k * 2 + 2] + self.seg_add[k * 2 + 2],
                    );
                }
            } else {
                self.add_to_range(a, b, value, k * 2 + 1, left, (left + right) / 2);
                self.add_to_range(a, b, value, k * 2 + 2, (left + right) / 2, right);
            }
        }

        pub fn update(&mut self, pos: usize, value: T) {
            let mut k = pos + self.num - 1;
            self.seg[k] = value;
            while k > 0 {
                k = (k - 1) / 2;
                self.seg[k] = (self.f.clone())(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
            }
        }

        pub fn get(&self, a: usize, b: usize) -> T {
            self.get_from_range(a, b, 0, 0, self.num)
        }

        fn get_from_range(&self, a: usize, b: usize, k: usize, left: usize, right: usize) -> T {
            if b <= left || right <= a {
                self.init
            } else if a <= left && right <= b {
                self.seg[k] + self.seg_add[k]
            } else {
                let mid = (left + right) / 2;
                let x = self.get_from_range(a, b, k * 2 + 1, left, mid);
                let y = self.get_from_range(a, b, k * 2 + 2, mid, right);
                (self.f.clone())(x, y) + self.seg_add[k]
            }
        }
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
