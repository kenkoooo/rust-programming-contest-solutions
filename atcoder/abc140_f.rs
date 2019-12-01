use std::collections::BTreeMap;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let s: Vec<u64> = sc.vec(1 << n);
    let mut map = BTreeMap::new();
    for s in s.into_iter() {
        *map.entry(s).or_insert(0) += 1;
    }

    let mut ans = vec![];
    let start = map.keys().next_back().cloned().unwrap();
    pop(&mut map, start);
    ans.push(start);
    while !map.is_empty() {
        let s = ans.len();
        for i in 0..s {
            let t = ans[i];
            match map.range(..t).next_back() {
                Some((&key, _)) => {
                    pop(&mut map, key);
                    ans.push(key);
                }
                _ => {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}

fn pop(map: &mut BTreeMap<u64, usize>, value: u64) {
    let count = map.get_mut(&value).unwrap();
    if *count == 1 {
        assert_eq!(map.remove(&value).unwrap(), 1);
    } else {
        *count -= 1;
    }
}

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
