fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let h: i64 = sc.read();
    let w: i64 = sc.read();
    let n: usize = sc.read();
    let r: i64 = sc.read();
    let c: i64 = sc.read();
    let s = sc.chars();
    let t = sc.chars();

    let check = |left: char, right: char, w: i64, c: i64| {
        let mut seg = vec![(0, 0); n + 1];
        let mut left_win = 0;
        let mut right_win = w + 1;
        for i in (0..n).rev() {
            if t[i] == left && right_win <= w {
                right_win += 1;
            }
            if t[i] == right && left_win > 0 {
                left_win -= 1;
            }
            if s[i] == left {
                left_win += 1;
            }
            if s[i] == right {
                right_win -= 1;
            }
            seg[i] = (left_win, right_win);
        }
        seg[n] = (0, w + 1);

        let mut left_cur = c;
        let mut right_cur = c;
        for (i, &(left_win, right_win)) in seg.iter().enumerate() {
            if left_cur <= left_win || right_win <= right_cur {
                return true;
            }
            if i == n {
                return false;
            }

            let (left_win, right_win) = seg[i + 1];
            if s[i] == left {
                left_cur -= 1;
            }
            if s[i] == right {
                right_cur += 1;
            }
            if t[i] == left && left_win < right_cur - 1 {
                right_cur -= 1;
            }
            if t[i] == right && left_cur + 1 < right_win {
                left_cur += 1;
            }
        }
        unreachable!()
    };

    if check('L', 'R', w, c) || check('U', 'D', h, r) {
        println!("NO");
    } else {
        println!("YES");
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
