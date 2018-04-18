use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut sc = Scanner::new();

    let x: i64 = sc.read();
    let y: i64 = sc.read();
    if (x - y).abs() <= 1 {
        println!("Brown");
    } else {
        println!("Alice");
    }
}

fn exp() {
    let mut map = BTreeMap::new();
    for z in 0..10000 {
        for x in 0..(z + 1) {
            let y = z - x;
            if y < x { continue; }
            if grundy(x, y, &mut map) == 0 {
                println!("{} {} {}", x, y, y - x);
            }
        }
    }
}

fn grundy(x: usize, y: usize, map: &mut BTreeMap<(usize, usize), usize>) -> usize {
    let (x, y) = norm(x, y);
    if !map.contains_key(&(x, y)) {
        let mut set = BTreeSet::new();
        for i in 1..(x / 2 + 1) {
            set.insert(grundy(x - 2 * i, y + i, map));
        }
        for i in 1..(y / 2 + 1) {
            set.insert(grundy(x + i, y - 2 * i, map));
        }

        let mut g = 0;
        loop {
            if set.contains(&g) {
                g += 1;
            } else {
                break;
            }
        }
        map.insert((x, y), g);
    }


    let g: usize = *map.get(&(x, y)).unwrap();
    return g;
}


fn norm(x: usize, y: usize) -> (usize, usize) {
    if x >= y { (y, x) } else { (x, y) }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

