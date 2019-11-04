use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let s: Vec<u64> = sc.vec(1 << n);

    let mut multiset = BTreeMap::new();
    for s in s.into_iter() {
        *multiset.entry(s).or_insert(0) += 1;
    }

    let max = *multiset.keys().next_back().unwrap();
    remove(&mut multiset, max);
    let mut ans = vec![max];
    while !multiset.is_empty() {
        let s = ans.len();
        for i in 0..s {
            let x = ans[i];
            match multiset.range(..x).next_back() {
                Some((&v, _)) => {
                    ans.push(v);
                    remove(&mut multiset, v);
                }
                None => {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}

fn remove(multiset: &mut BTreeMap<u64, usize>, v: u64) {
    let c = multiset.get_mut(&v).unwrap();
    if *c == 1 {
        assert_eq!(multiset.remove(&v), Some(1));
    } else {
        *c -= 1;
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
