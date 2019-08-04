use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let zero: u64 = sc.read();
    let one: u64 = sc.read();
    let g = match (zero % 2, one % 4) {
        (0, 0) | (0, 3) => 0,
        (0, 1) | (0, 2) => 1,
        (1, 0) => 0,
        (1, 1) | (1, 3) => 2,
        (1, 2) => 1,
        _ => unreachable!(),
    };
    if g != 0 {
        println!("Angel");
    } else {
        println!("Devil");
    }

    //    let mut map = BTreeMap::new();
    //    for i in 0..30 {
    //        for j in 0..30 {
    //            println!("{}\t{}\t{}", i, j, exp((i, j, true, true), &mut map));
    //        }
    //    }
}

type State = (u64, u64, bool, bool);

fn exp(state: State, map: &mut BTreeMap<State, u32>) -> u32 {
    if let Some(grundy) = map.get(&state) {
        return *grundy;
    }

    let (zero, one, is_first, is_even) = state;
    if zero == 0 && one == 0 {
        if is_even {
            return if is_first { 0 } else { 1 };
        } else {
            return if is_first { 1 } else { 0 };
        }
    }
    if is_first {
        let mut set = BTreeSet::new();
        if zero > 0 {
            set.insert(exp((zero - 1, one, false, is_even), map));
        }
        if one > 0 {
            set.insert(exp((zero, one - 1, false, !is_even), map));
        }
        let grundy = (0..).find(|i| !set.contains(i)).unwrap();
        map.insert(state, grundy);
        grundy
    } else {
        let mut set = BTreeSet::new();
        if zero > 0 {
            set.insert(exp((zero - 1, one, true, is_even), map));
        }
        if one > 0 {
            set.insert(exp((zero, one - 1, true, is_even), map));
        }
        let grundy = (0..).find(|i| !set.contains(i)).unwrap();
        map.insert(state, grundy);
        grundy
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
