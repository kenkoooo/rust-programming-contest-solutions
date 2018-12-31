use std::cmp;
use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let s: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut points = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                let x = i as i64;
                let y = j as i64;
                points.push((x, y));
            }
        }
    }

    let pairs = points.iter().map(|&(x, y)| (y - x, x)).collect::<Vec<_>>();
    let a1 = solve(h, pairs, false);

    let pairs = points.iter().map(|&(x, y)| (y + x, x)).collect::<Vec<_>>();
    let a2 = solve(h, pairs, true);

    let mut ans = a1 + a2;
    let mut map = BTreeMap::new();
    for &(x, y) in points.iter() {
        let k = y + x;
        map.entry(k).or_insert(Vec::new()).push((x, y));
    }
    for t in map.values() {
        let n = t.len();
        for i in 0..n {
            for j in 0..i {
                let (x1, y1) = t[j];
                let (x2, y2) = t[i];
                let d = (x1 - x2).abs();
                let v = vec![
                    (x1 - d, y1 - d),
                    (x2 - d, y2 - d),
                    (x1 + d, y1 + d),
                    (x2 + d, y2 + d),
                ];
                for &(x, y) in v.iter() {
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if x >= h || y >= w {
                        continue;
                    }
                    if s[x][y] == '#' {
                        ans -= 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

fn solve(h: usize, pairs: Vec<(i64, i64)>, s: bool) -> usize {
    let sign = if s { 1 } else { -1 };
    let mut map = BTreeMap::new();
    let mut list_map = BTreeMap::new();
    for &(k, x) in pairs.iter() {
        map.entry(k).or_insert(vec![0; h])[x as usize] = 1;
        list_map.entry(k).or_insert(Vec::new()).push(x);
    }
    for list in list_map.values_mut() {
        list.sort();
    }

    let sum_map: BTreeMap<_, _> = map
        .iter()
        .map(|(&k, v)| {
            let mut sum = vec![0; h + 1];
            for i in 0..h {
                sum[i + 1] = sum[i] + v[i];
            }
            (k, sum)
        })
        .collect();

    let mut ans = 0;
    for (k, list) in list_map.iter() {
        let n = list.len();
        for i in 0..n {
            for j in 0..i {
                let x1 = list[j];
                let x2 = list[i];
                let d = x2 - x1;
                assert!(d > 0);

                if let Some(sum) = sum_map.get(&(k - 2 * d * sign)) {
                    let from = cmp::max(x1 - d, 0) as usize;
                    let to = x1 as usize;
                    ans += sum[to + 1] - sum[from];
                }
                if let Some(sum) = sum_map.get(&(k + 2 * d * sign)) {
                    let from = x2 as usize;
                    let to = cmp::min(h - 1, (x2 + d) as usize);
                    ans += sum[to + 1] - sum[from];
                }
            }
        }
    }
    ans
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
