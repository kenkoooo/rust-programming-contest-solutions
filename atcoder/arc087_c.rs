use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let l: usize = sc.read();
    let mut trie = Trie::default();
    for _ in 0..n {
        let s = sc.chars();
        trie.add(s, 0);
    }

    let mut result = vec![];
    trie.traverse(l, &mut result);
    let result = result
        .into_iter()
        .map(|r| 1 << r.trailing_zeros())
        .fold(0, |xor, r| xor ^ r);
    println!("{}", if result != 0 { "Alice" } else { "Bob" });
}

struct Trie {
    zero: Option<Box<Trie>>,
    one: Option<Box<Trie>>,
}

impl Default for Trie {
    fn default() -> Trie {
        Trie {
            zero: None,
            one: None,
        }
    }
}
impl Trie {
    fn traverse(&self, l: usize, result: &mut Vec<usize>) {
        if self.zero.is_none() != self.one.is_none() {
            result.push(l);
        }
        if let Some(one) = self.one.as_ref() {
            one.traverse(l - 1, result);
        }
        if let Some(zero) = self.zero.as_ref() {
            zero.traverse(l - 1, result);
        }
    }

    fn add(&mut self, s: Vec<char>, pos: usize) {
        if s.len() == pos {
            return;
        }
        match s[pos] {
            '0' => {
                if self.zero.is_none() {
                    self.zero = Some(Box::new(Trie::default()));
                }
                self.zero.as_mut().unwrap().add(s, pos + 1);
            }
            '1' => {
                if self.one.is_none() {
                    self.one = Some(Box::new(Trie::default()));
                }
                self.one.as_mut().unwrap().add(s, pos + 1);
            }
            _ => unreachable!(),
        }
    }
}

fn grundy(h: usize, map: &mut BTreeMap<usize, usize>) -> usize {
    if map.contains_key(&h) {
        return *map.get(&h).unwrap();
    }
    let mut set = BTreeSet::new();
    for add in 1..(h + 1) {
        let mut g = 0;
        for r in 1..add {
            g ^= grundy(h - r, map);
        }
        set.insert(g);
    }

    let g = (0..).find(|x| !set.contains(x)).unwrap();
    map.insert(h, g);
    g
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
