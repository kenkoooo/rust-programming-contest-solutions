use std::collections::BTreeSet;

const INF: usize = 1e15 as usize;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let l: usize = sc.read();

    let mut trie = Trie::new();
    for _ in 0..n {
        let s = sc.chars();
        trie.add(&s, 0);
    }
    let mut count = vec![];
    trie.count_tree(0, &mut count);

    let mut ans = 0;
    for grundy in count.iter().map(|&depth| (l - depth).trailing_zeros() + 1) {
        ans ^= grundy;
    }
    println!("{}", if ans == 0 { "Bob" } else { "Alice" });

    // let mut grundy = vec![INF; 31];
    // grundy[0] = 0;
    // get(30, &mut grundy);
    // println!("{:?}", grundy);
}

struct Trie {
    child: Vec<Option<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            child: vec![None, None],
        }
    }

    fn count_tree(&self, depth: usize, result: &mut Vec<usize>) {
        let len = self.child.iter().filter(|c| c.is_some()).count();
        if len == 1 {
            result.push(depth);
        }
        for child in self.child.iter() {
            match child {
                Some(child) => child.count_tree(depth + 1, result),
                None => {}
            }
        }
    }

    fn add(&mut self, s: &Vec<char>, pos: usize) {
        if s.len() == pos {
            return;
        }
        let index = s[pos] as usize - ('0' as usize);
        self.child[index].get_or_insert(Trie::new()).add(s, pos + 1);
    }
}

fn get(i: usize, grundy: &mut Vec<usize>) -> usize {
    if grundy[i] != INF {
        return grundy[i];
    }

    let mut set = BTreeSet::new();
    for k in 1..(i + 1) {
        let mut g = 0;
        for j in k..i {
            g ^= get(j, grundy);
        }
        set.insert(g);
    }
    for j in 0.. {
        if !set.contains(&j) {
            grundy[i] = j;
            break;
        }
    }
    grundy[i]
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
