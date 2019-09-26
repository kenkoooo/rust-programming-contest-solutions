use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = (0..n).map(|_| sc.read::<usize>() - 1).collect();
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[a[i]] = i;
    }

    let mut ans = 0;
    let mut large_set = BTreeSet::new();
    for i in (0..n).rev() {
        let pos = pos[i];
        let mut left_iter = large_set.range(pos..);
        let left_first = left_iter.next().cloned();
        let left_second = left_iter.next().cloned();

        let mut right_iter = large_set.range(..pos);
        let right_first = right_iter.next_back().cloned();
        let right_second = right_iter.next_back().cloned();

        //        eprintln!(
        //            "{:?}",
        //            (i, right_second, right_first, left_first, left_second)
        //        );
        // left
        if let Some(first) = left_first {
            let right = match right_first {
                Some(first) => pos - first,
                None => pos + 1,
            };
            let left = match left_second {
                Some(second) => second - first,
                None => n - first,
            };
            //            eprintln!("left={} right={} i={}", left, right, i);
            ans += right * left * (i + 1);
        }

        if let Some(first) = right_first {
            let left = match left_first {
                Some(first) => first - pos,
                None => n - pos,
            };
            let right = match right_second {
                Some(second) => first - second,
                None => first + 1,
            };
            //            eprintln!("left={} right={} i={}", left, right, i);
            ans += right * left * (i + 1);
        }

        large_set.insert(pos);
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
