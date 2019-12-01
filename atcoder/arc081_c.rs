use std::cmp;
use std::collections::{BTreeSet, BinaryHeap, VecDeque};

const INF: i64 = 1e9 as i64;
fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let a = sc.chars();
    let n = a.len();
    let mut heads = vec![];
    let mut set = BTreeSet::new();
    for (i, &c) in a.iter().enumerate().rev() {
        set.insert(c);
        if set.len() == 26 {
            set.clear();
            heads.push(i);
        }
    }

    let mut group = vec![0; n];
    for (i, head) in heads.into_iter().rev().enumerate() {
        group[head] = i + 1;
    }
    for i in 1..n {
        group[i] = cmp::max(group[i], group[i - 1]);
    }

    let mut index = vec![VecDeque::new(); 26];
    for i in 0..n {
        let c = a[i] as usize - 'a' as usize;
        index[c].push_back(i);
    }

    let mut cur_group = 0;
    let mut cur_pos = 0;
    let mut ans = String::new();
    loop {
        for i in 0..26 {
            while let Some(x) = index[i].pop_front() {
                if x >= cur_pos {
                    index[i].push_front(x);
                    break;
                }
            }
            let c = (i as u8 + 'a' as u8) as char;
            match index[i].iter().next().cloned() {
                Some(head) => {
                    if group[head] == cur_group + 1 {
                        ans.push(c);
                        cur_pos = head + 1;
                        cur_group += 1;
                        break;
                    }
                }
                None => {
                    println!("{}{}", ans, c);
                    return;
                }
            }
        }
    }
}
/// frqn
/// vhydscshfcgdemurlfrutcpzhopfotpifgepnqjxupnsk(a)pziurswqazdwnwbgdhyktfyhqqxpoidf
/// hjdakoxraiedxskywuepzfniuyskxiyjpjlxuqnfgmnj(c)vtlpnclfkpervxmdbvrbrdn

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
