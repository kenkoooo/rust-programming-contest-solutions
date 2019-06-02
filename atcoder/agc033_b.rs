fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let n: usize = sc.read();
    let r: usize = sc.read();
    let c: usize = sc.read();
    let s: Vec<char> = sc.chars();
    let t: Vec<char> = sc.chars();

    let check = |left: char, right: char, w: usize, cur: usize| -> bool {
        let mut right_win = w + 1;
        let mut left_win = 0;
        let mut segments = vec![(0, 0); n];
        for i in (0..n).rev() {
            if t[i] == left && right_win < w + 1 {
                right_win += 1;
            }
            if t[i] == right && left_win > 0 {
                left_win -= 1;
            }

            if s[i] == left {
                left_win += 1;
            }
            if s[i] == right && right_win > 0 {
                right_win -= 1;
            }
            segments[i] = (left_win, right_win);
        }
        segments.push((0, w + 1));

        let mut left_cur = cur;
        let mut right_cur = cur;

        for i in 0..n {
            let (left_win, right_win) = segments[i];
            if left_cur <= left_win || right_win <= right_cur {
                return true;
            }
            if s[i] == left {
                left_cur -= 1;
            }
            if s[i] == right {
                right_cur += 1;
            }

            let (left_win, right_win) = segments[i + 1];
            if t[i] == left && right_cur - 1 > left_win {
                right_cur -= 1;
            }
            if t[i] == right && left_cur + 1 < right_win {
                left_cur += 1;
            }
        }

        if left_cur <= 0 || w + 1 <= right_cur {
            true
        } else {
            false
        }
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
