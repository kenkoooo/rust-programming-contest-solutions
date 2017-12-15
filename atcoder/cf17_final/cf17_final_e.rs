use std::io;
use std::cmp;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct MinSize {
    value: usize,
}

impl Ord for MinSize {
    fn cmp(&self, other: &MinSize) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for MinSize {
    fn partial_cmp(&self, other: &MinSize) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let s = read_line().trim().to_owned();
    let n = s.len();
    let methods = read_values::<usize>()[0];
    let mut lr = Vec::new();
    for _ in 0..methods {
        let v = read_values::<usize>();
        let mut l: usize = v[0] - 1;
        let mut r: usize = v[1] - 1;
        let half = n >> 1;

        if (n & 1) == 1 {
            if l == half && r == half {
                continue;
            }
            if half < l {
                l -= 1;
            }
            if half <= r {
                r -= 1;
            }
        }
        lr.push((l, r));
    }

    let s = {
        let t = s.bytes().map(|c| {
            c - "a".bytes().next().unwrap()
        }).collect::<Vec<_>>();
        let mut polished = Vec::new();
        for i in 0..n {
            if (n & 1) != 1 || (n >> 1) != i {
                polished.push(t[i]);
            }
        }
        polished
    };
    let n = s.len();

    let mut right_map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for x in lr.iter() {
        let (l, r) = *x;

        let half = n >> 1;
        let (l, r) = if r < half {
            (l, r)
        } else if l >= half {
            (n - 1 - r, n - 1 - l)
        } else {
            let right_side = r - (half - 1);
            let left_side = half - l;
            if right_side > left_side {
                let tmp_l = half + left_side;
                (n - 1 - r, n - 1 - tmp_l)
            } else if right_side == left_side {
                continue;
            } else {
                (l, half - 1 - right_side)
            }
        };

        if !right_map.contains_key(&l) {
            right_map.insert(l, BTreeSet::new());
        }
        right_map.get_mut(&l).unwrap().insert(r);
    }

    let mut left_map = BTreeMap::new();
    for (left, rights) in right_map.iter() {
        for r in rights.iter() {
            if !left_map.contains_key(&(r + 1)) {
                left_map.insert(r + 1, BTreeSet::new());
            }
            left_map.get_mut(&(r + 1)).unwrap().insert(*left);
        }
    }

    let mut merged_map = BTreeMap::new();
    for left in 0..n {
        if !right_map.contains_key(&left) {
            continue;
        }

        let mut lefts = BTreeSet::new();

        let mut heap = BinaryHeap::new();
        heap.push(MinSize { value: left });
        let mut right = left;
        while !heap.is_empty() {
            let v = heap.pop().unwrap().value;
            lefts.insert(v);

            if let Some(set) = right_map.get(&v) {
                for r in set.iter() {
                    heap.push(MinSize { value: *r + 1 });
                    right = cmp::max(right, *r + 1);
                }
            }
            right_map.remove(&v);

            if let Some(set) = left_map.get(&v) {
                for l in set.iter() {
                    heap.push(MinSize { value: *l });
                }
            }
            left_map.remove(&v);
        }
        lefts.insert(right);

        let list = lefts.iter().map(|x| x.clone()).collect::<Vec<_>>();
        for i in 0..(list.len() - 1) {
            let cur = list[i];
            let next = list[i + 1];
            merged_map.insert(cur, next);
        }
    }

    let mut imos = vec![0; n / 2 + 1];
    let mut cur = 0;
    for i in 0..(n / 2) {
        cur += imos[i];
        let from = (s[i] as i64 + cur) % 26;
        let to = (s[n - 1 - i] as i64) % 26;
        let add = (to + 26 - from) % 26;
        if add == 0 {
            continue;
        }
        if !merged_map.contains_key(&i) {
            println!("NO");
            return;
        }
        let x = *merged_map.get(&i).unwrap();
        cur += add;
        imos[x] -= add;
    }
    println!("YES");
}

struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| { i }).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx == fy {
            return false;
        }

        let (tx, ty) = if self.sizes[fx] < self.sizes[fy] {
            (fy, fx)
        } else {
            (fx, fy)
        };

        self.parent[ty] = tx;
        self.sizes[tx] += self.sizes[ty];
        self.sizes[ty] = 0;
        self.size -= 1;
        return true;
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}