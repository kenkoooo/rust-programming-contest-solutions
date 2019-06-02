use std::cmp;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let s: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();

    let mut positive_map = BTreeMap::new();
    let mut negative_map = BTreeMap::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }
            let i = i as i64;
            let j = j as i64;
            let k = j - i;
            let l = j + i;
            positive_map.entry(k).or_insert(Vec::new()).push(j);
            negative_map.entry(l).or_insert(Vec::new()).push(j);
        }
    }

    for v in positive_map.values_mut() {
        v.sort();
    }
    for v in negative_map.values_mut() {
        v.sort();
    }

    let positive_sum = positive_map
        .iter()
        .map(|(k, v)| {
            let mut value: Vec<usize> = vec![0; w];
            for &v in v.iter() {
                value[v as usize] = 1;
            }
            let mut sum = vec![0; w + 1];
            for i in 0..w {
                sum[i + 1] = sum[i] + value[i];
            }
            (*k, sum)
        })
        .collect::<BTreeMap<_, _>>();
    let negative_sum = negative_map
        .iter()
        .map(|(k, v)| {
            let mut value: Vec<usize> = vec![0; w];
            for &v in v.iter() {
                value[v as usize] = 1;
            }
            let mut sum = vec![0; w + 1];
            for i in 0..w {
                sum[i + 1] = sum[i] + value[i];
            }
            (*k, sum)
        })
        .collect::<BTreeMap<_, _>>();

    let mut ans = 0;
    for (k, v) in positive_map.iter() {
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                let (x1, y1) = (v[i] - *k, v[i]);
                let (x2, y2) = (v[j] - *k, v[j]);
                assert!(x1 < x2);
                assert!(y1 < y2);
                assert_eq!(x2 - x1, y2 - y1);
                let d = x2 - x1;

                if let Some(sum) = positive_sum.get(&(k + d * 2)) {
                    let y3_from = y2;
                    let y3_to = cmp::min(w as i64, y2 + d + 1);
                    ans += sum[y3_to as usize] - sum[y3_from as usize];
                }
                if let Some(sum) = positive_sum.get(&(k - d * 2)) {
                    let y3_from = cmp::max(0, y1 - d);
                    let y3_to = y1 + 1;
                    ans += sum[y3_to as usize] - sum[y3_from as usize];
                }
            }
        }
    }
    for (l, v) in negative_map.iter() {
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                let (x1, y1) = (*l - v[i], v[i]);
                let (x2, y2) = (*l - v[j], v[j]);
                assert!(x1 > x2);
                assert!(y1 < y2);
                assert_eq!(x1 - x2, y2 - y1);
                let d = x1 - x2;

                if let Some(sum) = negative_sum.get(&(l - d * 2)) {
                    let y3_from = cmp::max(0, y1 - d);
                    let y3_to = y1 + 1;
                    ans += sum[y3_to as usize] - sum[y3_from as usize];
                }
                if let Some(sum) = negative_sum.get(&(l + d * 2)) {
                    let y3_from = cmp::max(0, y2);
                    let y3_to = cmp::min(w as i64, y2 + d + 1);
                    ans += sum[y3_to as usize] - sum[y3_from as usize];
                }
            }
        }
    }

    // let mut count = 0;
    // let mut set = BTreeSet::new();
    for (l, v) in negative_map.iter() {
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                let (x1, y1) = (*l - v[i], v[i]);
                let (x2, y2) = (*l - v[j], v[j]);
                assert!(x1 > x2);
                assert!(y1 < y2);
                assert_eq!(x1 - x2, y2 - y1);
                let d = x1 - x2;

                for &(x3, y3) in [
                    (x1 - d, y1 - d),
                    (x2 - d, y2 - d),
                    (x1 + d, y1 + d),
                    (x2 + d, y2 + d),
                ]
                .into_iter()
                {
                    if 0 <= x3
                        && x3 < (h as i64)
                        && 0 <= y3
                        && y3 < (w as i64)
                        && s[x3 as usize][y3 as usize] == '#'
                    {
                        assert_eq!(s[x1 as usize][y1 as usize], '#');
                        assert_eq!(s[x2 as usize][y2 as usize], '#');
                        assert_eq!(
                            (x1 - x2).abs() + (y1 - y2).abs(),
                            (x1 - x3).abs() + (y1 - y3).abs()
                        );
                        assert_eq!(
                            (x1 - x2).abs() + (y1 - y2).abs(),
                            (x2 - x3).abs() + (y2 - y3).abs()
                        );
                        // let v = [(x1, y1), (x2, y2), (x3, y3)]
                        //     .into_iter()
                        //     .cloned()
                        //     .collect::<BTreeSet<_>>();
                        // set.insert(v);
                        // count += 1;
                        ans -= 1;
                    }
                }
            }
        }
    }

    // assert_eq!(count, set.len());

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
