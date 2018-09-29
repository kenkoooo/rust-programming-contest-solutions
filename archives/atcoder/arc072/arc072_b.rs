use std::collections::{BTreeMap, BTreeSet};
fn main() {
    let mut sc = Scanner::new();
    let x: u64 = sc.read();
    let y: u64 = sc.read();
    let (x, y) = if x > y { (y, x) } else { (x, y) };
    println!("{}", if y - x <= 1 { "Brown" } else { "Alice" });
    // exp();
}

fn exp() {
    let mut map = BTreeMap::new();
    map.insert((0, 0), 0);
    map.insert((0, 1), 0);
    map.insert((1, 1), 0);

    for i in 0..1000 {
        for j in i..1000 {
            grundy(i, j, &mut map);
        }
    }
}

fn grundy(x: usize, y: usize, map: &mut BTreeMap<(usize, usize), usize>) -> usize {
    let (x, y) = if x > y { (y, x) } else { (x, y) };

    if map.contains_key(&(x, y)) {
        return map[&(x, y)];
    }

    let mut grundy_nums = BTreeSet::new();
    for x_to_y in 1..(x / 2 + 1) {
        let next_x = x - x_to_y * 2;
        let next_y = y + x_to_y;
        grundy_nums.insert(grundy(next_x, next_y, map));
    }
    for y_to_x in 1..(y / 2 + 1) {
        let next_x = x + y_to_x;
        let next_y = y - y_to_x * 2;
        grundy_nums.insert(grundy(next_x, next_y, map));
    }

    let mut g = 0;
    loop {
        if !grundy_nums.contains(&g) {
            map.insert((x, y), g);
            if g == 0 {
                println!("{} {}", x, y);
            }
            return g;
        }
        g += 1;
    }
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
