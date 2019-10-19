use std::cmp;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let l: i64 = sc.read();
    let x: i64 = sc.read();

    let mut set = BTreeSet::new();
    set.insert(x);
    let mut min = BTreeMap::new();
    min.insert(cmp::min(x, l - x), 1);
    let mut diff = BTreeMap::new();
    for _ in 0..n {
        let a: i64 = sc.read();
        let m = cmp::min(a, l - a);
        if set.len() >= 2 {
            let (max2, max) = get_max_2(&min);
            let corner = ((l - max2) - max).abs();
            remove(&mut diff, corner);
        }
        if set.contains(&a) {
            let cur_count = *min.get(&m).unwrap();
            let (m0, m1) = get_prev_2(m - 1, &min);
            let (m2, m3) = get_next_2(m + 1, &min);
            let (m0, m1, m2, m3) = if cur_count == 2 {
                (m0, m1, Some(m), m2)
            } else {
                (m0, m1, m2, m3)
            };

            // m0, m1, m, m2, m3
            // m-m0, m2-m1, m3-m
            // m2-m0, m3-m1
            if let Some(m0) = m0 {
                remove(&mut diff, m - m0);
            }
            if let (Some(m2), Some(m1)) = (m2, m1) {
                remove(&mut diff, m2 - m1);
            }
            if let Some(m3) = m3 {
                remove(&mut diff, m3 - m);
            }
            if let (Some(m2), Some(m0)) = (m2, m0) {
                insert(&mut diff, m2 - m0);
            }
            if let (Some(m3), Some(m1)) = (m3, m1) {
                insert(&mut diff, m3 - m1);
            }
            let removed = set.remove(&a);
            assert!(removed);
            remove(&mut min, m);
        } else {
            let cur_count = min.get(&m).cloned().unwrap_or(0);
            let (m0, m1) = get_prev_2(m - 1, &min);
            let (m2, m3) = get_next_2(m + 1, &min);
            let (m0, m1, m2, m3) = if cur_count == 1 {
                (m0, m1, Some(m), m2)
            } else {
                assert_eq!(cur_count, 0);
                (m0, m1, m2, m3)
            };

            // m0, m1, (m), m2, m3
            // m2-m0, m3-m1
            // m-m0, m2-m1, m3-m

            if let (Some(m2), Some(m0)) = (m2, m0) {
                remove(&mut diff, m2 - m0);
            }
            if let (Some(m3), Some(m1)) = (m3, m1) {
                remove(&mut diff, m3 - m1);
            }
            if let Some(m0) = m0 {
                insert(&mut diff, m - m0);
            }
            if let (Some(m2), Some(m1)) = (m2, m1) {
                insert(&mut diff, m2 - m1);
            }
            if let Some(m3) = m3 {
                insert(&mut diff, m3 - m);
            }
            set.insert(a);
            insert(&mut min, m);
        }
        let (max2, max) = get_max_2(&min);
        let corner = ((l - max2) - max).abs();
        insert(&mut diff, corner);

        //        eprintln!("{:?}", set);
        //        eprintln!("{:?}", min);
        //        eprintln!("{:?}", diff);
        let ans = diff.keys().next().cloned().unwrap();
        //        assert_eq!(ans, flat_min(&set, l));
        println!("{}", ans);
    }
}

fn get_max_2(map: &BTreeMap<i64, usize>) -> (i64, i64) {
    let mut iter = map.iter();
    let (&max, &count) = iter.next_back().unwrap();
    if count == 2 {
        (max, max)
    } else {
        let (&max2, _) = iter.next_back().unwrap();
        (max2, max)
    }
}

fn flat_min(set: &BTreeSet<i64>, l: i64) -> i64 {
    let mut v = set.iter().map(|&v| cmp::min(v, l - v)).collect::<Vec<_>>();
    v.sort();
    v = v
        .into_iter()
        .enumerate()
        .map(|(i, v)| if i % 2 == 0 { l - v } else { v })
        .collect();
    v.sort();
    (1..v.len()).map(|i| v[i] - v[i - 1]).min().unwrap()
}

fn insert(map: &mut BTreeMap<i64, usize>, v: i64) {
    *map.entry(v).or_insert(0) += 1;
}
fn remove(map: &mut BTreeMap<i64, usize>, v: i64) {
    let count = *map.get(&v).unwrap();
    if count > 1 {
        map.insert(v, count - 1);
    } else {
        let removed = map.remove(&v);
        assert_eq!(removed, Some(1));
    }
}

fn get_prev_2(a: i64, min: &BTreeMap<i64, usize>) -> (Option<i64>, Option<i64>) {
    let mut prev = min.range(..(a + 1));
    if let Some((&a1, &count)) = prev.next_back() {
        if count == 2 {
            (Some(a1), Some(a1))
        } else {
            match prev.next_back() {
                Some((&a0, _)) => (Some(a0), Some(a1)),
                None => (None, Some(a1)),
            }
        }
    } else {
        (None, None)
    }
}

fn get_next_2(a: i64, min: &BTreeMap<i64, usize>) -> (Option<i64>, Option<i64>) {
    let mut next = min.range(a..);
    if let Some((&a2, &count)) = next.next() {
        if count == 2 {
            (Some(a2), Some(a2))
        } else {
            assert_eq!(count, 1);
            match next.next() {
                Some((&a3, _)) => (Some(a2), Some(a3)),
                None => (Some(a2), None),
            }
        }
    } else {
        (None, None)
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
