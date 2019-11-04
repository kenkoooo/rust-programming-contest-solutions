fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 0..n {
        if graph[i]
            .iter()
            .filter(|&&child| graph[child].len() == 1)
            .count()
            >= 2
        {
            println!("First");
            return;
        }
    }
    println!("Second");
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
