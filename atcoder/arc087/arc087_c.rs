use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let l: usize = sc.read();
    let mut trie = TrieNode::new();
    for _ in 0..n {
        let s: Vec<usize> = sc
            .chars()
            .into_iter()
            .map(|c| c as usize - '0' as usize)
            .collect();
        trie.add(&s, 0);
    }

    let mut result = vec![];
    trie.get_count(0, &mut result);

    let mut grundy: u64 = 0;
    for &r in result.iter() {
        let height = l - r;
        let g = 1 << (height as u32).trailing_zeros();
        grundy ^= g;
    }
    println!("{}", if grundy != 0 { "Alice" } else { "Bob" });
}

struct TrieNode {
    child: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            child: [None, None],
        }
    }

    fn add(&mut self, s: &[usize], pos: usize) {
        if s.len() == pos {
            return;
        }
        if self.child[s[pos]].is_none() {
            self.child[s[pos]] = Some(Box::new(TrieNode::new()));
        }
        self.child[s[pos]].as_mut().unwrap().add(s, pos + 1);
    }

    fn get_count(&self, height: usize, result: &mut Vec<usize>) {
        let count = self.child.iter().filter(|c| c.is_none()).count();
        if count == 1 {
            result.push(height);
        }
        for child in self.child.iter() {
            match child.as_ref() {
                Some(child) => child.get_count(height + 1, result),
                None => {}
            }
        }
    }
}

fn grundy(h: usize, map: &mut BTreeMap<usize, u32>) -> u32 {
    match map.get(&h) {
        Some(&v) => v,
        None if h == 1 => 1,
        None => {
            let mut set = BTreeSet::new();
            for remove in 1..(h + 1) {
                let mut g = 0;
                for i in 1..remove {
                    g ^= grundy(h - i, map);
                }
                set.insert(g);
            }
            let g = (0..).skip_while(|i| set.contains(&i)).next().unwrap();
            map.insert(h, g);
            g
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
