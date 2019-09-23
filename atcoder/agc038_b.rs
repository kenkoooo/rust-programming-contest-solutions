use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let k: usize = sc.read();
    let p: Vec<u64> = sc.vec(n);
    let mut set = BTreeSet::new();
    let mut same_as_next = vec![];
    for i in 0..n {
        if set.len() < k {
            set.insert(p[i]);
            continue;
        }
        let max = *set.iter().next_back().unwrap();
        let min = *set.iter().next().unwrap();
        if max < p[i] && min == p[i - k] {
            same_as_next.push(i - k);
            //            eprintln!("{:?}", set);
        }
        set.insert(p[i]);
        set.remove(&p[i - k]);
    }

    let mut same_as_nothing = BTreeSet::new();
    let mut i = 0;
    while i < n {
        let mut to = i;
        for j in (i + 1)..n {
            if p[j - 1] < p[j] {
                to = j;
            } else {
                break;
            }
        }
        for from in i.. {
            if from + k - 1 > to {
                break;
            }
            same_as_nothing.insert(from);
        }
        i = to + 1;
    }

    //    eprintln!("{:?}", same_as_next);
    //    eprintln!("{:?}", same_as_nothing);
    let ignore1 = same_as_next
        .into_iter()
        .filter(|i| !same_as_nothing.contains(i))
        .count();
    let ignore2 = if same_as_nothing.is_empty() {
        0
    } else {
        same_as_nothing.len() - 1
    };
    println!("{}", n - k + 1 - ignore1 - ignore2);
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
