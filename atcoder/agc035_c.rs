fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();

    if n == 1 || n == 2 || (n % 4 == 0 && n.count_ones() == 1) {
        println!("No");
        return;
    } else if n == 3 {
        println!("Yes");
        println!("1 2");
        println!("2 3");
        println!("3 4");
        println!("4 5");
        println!("5 6");
        return;
    }

    let w = (n + 1) / 4 * 4 - 1;
    let mut used = vec![false; n];
    let mut ans = vec![];
    let mut row = vec![];
    row.push(w);
    row.push(1);
    if w != 3 {
        row.push(3);
    }
    row.push(2);
    if n % 4 == 0 {
        let mut head = 0;
        let mut tail = 0;
        for i in 0..30 {
            if (1 << i) & n != 0 {
                used[1 << i] = true;
                row.push(1 << i);
                if head == 0 {
                    head = 1 << i;
                }
                tail = 1 << i;
            }
        }
        ans.push((head, n));
        ans.push((tail, n + n));
    }
    for i in 4..w {
        if used[i] {
            continue;
        }
        row.push(i);
    }
    let s = row.len();
    for i in 1..s {
        ans.push((row[i - 1], row[i]));
        ans.push((row[i - 1] + n, row[i] + n));
    }

    ans.push((row[s - 1], row[0] + n));
    if w + 2 <= n {
        ans.push((w + 1, 1));
        ans.push((w + 1, w + 2));

        ans.push((w + 2 + n, 1));
        ans.push((w + 2 + n, w + 1 + n));
    }
    if w + 3 <= n {
        ans.push((w + 3, w + 1));
        ans.push((w + 3 + n, 3));
    }

    println!("Yes");
    for (i, j) in ans.into_iter() {
        println!("{} {}", i, j);
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
