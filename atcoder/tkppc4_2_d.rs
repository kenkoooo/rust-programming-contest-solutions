use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let p: i64 = sc.read();
    let q: i64 = sc.read();
    let a: Vec<i64> = sc.vec(n);

    if (p + q).abs() % 2 != 0 {
        println!("0");
        return;
    }
    let ax = (p + q) / 2;
    let ayz = p - ax;

    let mut z_map: BTreeMap<_, u64> = BTreeMap::new();
    for &a in a.iter() {
        *z_map.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    let mut ax_count = 0;
    for i in 0..n {
        // remove
        match z_map.get(&a[i]).cloned() {
            Some(c) => {
                if c == 1 {
                    z_map.remove(&a[i]);
                } else {
                    z_map.insert(a[i], c - 1);
                }
            }
            None => {}
        }

        // calc
        let ay = a[i];
        let az = ayz - ay;
        let az_count = z_map.get(&az).cloned().unwrap_or(0);
        ans += ax_count * az_count;

        if a[i] == ax {
            ax_count += 1;
        }
    }

    println!("{}", ans);
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
