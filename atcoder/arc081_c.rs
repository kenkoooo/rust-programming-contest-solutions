const LETTER_NUM: usize = 26;
fn main() {
    let mut sc = Scanner::new();
    let s: Vec<usize> = sc.read::<String>()
        .chars()
        .map(|c| ((c as u8) - ('a' as u8)) as usize)
        .collect();
    let mut indices = vec![vec![]; LETTER_NUM];
    for i in 0..s.len() {
        indices[s[i]].push(i + 1);
    }

    let mut cur = Vec::new();
    for depth in 1..1000 {
        if dfs(&indices, &mut cur, depth) {
            for &c in &cur {
                let c = (c as u8 + ('a' as u8)) as char;
                print!("{}", c);
            }
            println!();
            return;
        }
    }
}

fn dfs(indices: &Vec<Vec<usize>>, cur: &mut Vec<usize>, max_depth: usize) -> bool {
    if cur.len() == max_depth {
        let mut cur_i = 0;
        for &cur in cur.iter() {
            let next_index = match indices[cur].binary_search(&(cur_i + 1)) {
                Ok(i) => i,
                Err(i) => i,
            };
            if next_index == indices[cur].len() {
                return true;
            }
            cur_i = indices[cur][next_index];
        }
        return false;
    }

    for i in 0..LETTER_NUM {
        cur.push(i);
        if dfs(indices, cur, max_depth) {
            return true;
        }
        cur.pop();
    }
    return false;
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
