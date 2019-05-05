fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.read();
    let _: usize = sc.read();

    let s: Vec<Vec<char>> = (0..h)
        .map(|_| sc.read::<String>().chars().collect())
        .collect();

    let mut rows = vec![0; h];
    let mut used = vec![false; h];

    if solve(&s, &mut rows, &mut used, 0) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn solve(s: &Vec<Vec<char>>, rows: &mut Vec<usize>, used: &mut Vec<bool>, pos: usize) -> bool {
    let h = rows.len();
    if pos == h / 2 {
        if h % 2 == 0 {
            return check(s, rows);
        } else {
            for i in 0..h {
                if !used[i] {
                    rows[pos] = i;
                    break;
                }
            }
            return check(s, rows);
        }
    }

    for i in 0..h {
        if used[i] {
            continue;
        }
        used[i] = true;
        for j in (i + 1)..h {
            if used[j] {
                continue;
            }
            used[j] = true;

            rows[pos] = i;
            rows[h - 1 - pos] = j;
            if solve(s, rows, used, pos + 1) {
                return true;
            }

            used[j] = false;
        }
        used[i] = false;
    }

    return false;
}

fn check(s: &Vec<Vec<char>>, rows: &Vec<usize>) -> bool {
    let h = s.len();
    let w = s[0].len();
    let mut centered = false;
    let mut used = vec![false; w];
    for i in 0..w {
        if used[i] {
            continue;
        }
        for j in (i + 1)..w {
            if used[i] {
                continue;
            }

            let mut ok = true;
            for k in 0..h {
                if s[rows[k]][i] != s[rows[h - 1 - k]][j] {
                    ok = false;
                    break;
                }
            }

            if ok {
                used[i] = true;
                used[j] = true;
                break;
            }
        }
        if !used[i] {
            if w % 2 == 0 {
                return false;
            }
            if centered {
                return false;
            }
            centered = true;
            for k in 0..h {
                if s[rows[k]][i] != s[rows[h - 1 - k]][i] {
                    return false;
                }
            }
        }
    }

    return true;
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
