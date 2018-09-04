use std::collections::BTreeMap;

const INF: i64 = 1e15 as i64;

fn main() {
    let mut sc = Scanner::new();
    let week_num: i64 = sc.read();
    let day_num: i64 = sc.read();
    let n = sc.read();
    let m = sc.read();
    let distance: i64 = sc.read();
    let a: Vec<i64> = sc.read_vec(n);
    let mut holidays = vec![vec![]; day_num as usize + 1];
    for _ in 0..m {
        let b: i64 = sc.read();
        let c: usize = sc.read();
        holidays[c].push(b);
    }

    let mut map = BTreeMap::new();
    map.insert(INF, 1);
    map.insert(-INF, 1);

    let mut ans = 0;
    for &a in a.iter() {
        ans += map.insert_holiday(a, distance);
    }

    for c in 0..(day_num + 1) {
        for &b in holidays[c as usize].iter() {
            let day = (b - 1) * day_num + c;
            ans += map.insert_holiday(day, distance);
        }
    }

    for d in 1..(day_num + 1) {
        println!("{}", ans);
        for &a in a.iter() {
            ans -= map.remove_holiday(a + d - 1, distance);
            ans += map.insert_holiday(a + d, distance);
        }
        for &b in holidays[d as usize].iter() {
            let day = (b - 1) * day_num + d;
            ans -= map.remove_holiday(day, distance);
            ans += map.insert_holiday(day + day_num, distance);
        }
    }
}

trait HolidayMap {
    fn insert_holiday(&mut self, day: i64, distance: i64) -> i64;
    fn remove_holiday(&mut self, day: i64, distance: i64) -> i64;
}

fn get_before_after(map: &mut BTreeMap<i64, usize>, day: i64) -> (i64, i64) {
    let (&before, _) = map.range(..day).next_back().unwrap();
    let (&after, _) = map.range((day + 1)..).next().unwrap();
    (before, after)
}
fn get_new_holidays(map: &mut BTreeMap<i64, usize>, day: i64, distance: i64) -> i64 {
    let (before, after) = get_before_after(map, day);
    let before2after = after - before - 1;
    assert!(before2after >= 1);

    let mut ans = 0;
    if before2after > distance {
        let before = day - before - 1;
        let after = after - day - 1;
        if before <= distance {
            ans += before;
        }
        if after <= distance {
            ans += after;
        }
        ans += 1;
    }
    ans
}

impl HolidayMap for BTreeMap<i64, usize> {
    fn remove_holiday(&mut self, day: i64, distance: i64) -> i64 {
        let cur = {
            let ptr = self.get_mut(&day).unwrap();
            let cur = *ptr;
            assert!(cur > 0);
            *ptr -= 1;
            cur
        };
        if cur == 1 {
            self.remove(&day);
            get_new_holidays(self, day, distance)
        } else {
            0
        }
    }

    fn insert_holiday(&mut self, day: i64, distance: i64) -> i64 {
        let cur = {
            let ptr: &mut usize = self.entry(day).or_insert(0);
            let cur = *ptr;
            *ptr += 1;
            cur
        };
        if cur == 0 {
            get_new_holidays(self, day, distance)
        } else {
            0
        }
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
