use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let a: Vec<u64> = sc.vec(n);
    if n == 2 {
        if a[0] == a[1] {
            println!("YES");
        } else {
            println!("NO");
        }
        return;
    }
    let mut tree = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        tree[a].push(b);
        tree[b].push(a);
    }

    for i in 0..n {
        if tree[i].len() > 1 {
            match dfs(i, i, &tree, &a) {
                Some(up) if up == 0 => println!("YES"),
                _ => println!("NO"),
            }
            return;
        }
    }
}

fn dfs(v: usize, p: usize, tree: &Vec<Vec<usize>>, a: &Vec<u64>) -> Option<u64> {
    if tree[v].len() == 1 {
        return Some(a[v]);
    }
    let mut sum = 0;
    let mut max = 0;
    for &next in tree[v].iter() {
        if p == next {
            continue;
        }
        match dfs(next, v, tree, a) {
            Some(next) => {
                sum += next;
                max = cmp::max(max, next);
            }
            None => {
                return None;
            }
        }
    }
    if 2 * a[v] < sum {
        return None;
    }
    let up = 2 * a[v] - sum;
    if sum < up {
        return None;
    }
    let connecting = (sum - up) / 2;
    if connecting + up < max {
        return None;
    }
    Some(up)
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
