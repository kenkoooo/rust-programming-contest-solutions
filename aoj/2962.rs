use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc.chars();
    let result = parse(&s);
    println!("{} {}", result.0, result.1);
}

fn read_char(_: &Vec<char>, pos: &mut usize) {
    *pos += 1;
}

fn next_char(s: &Vec<char>, pos: &mut usize) -> char {
    s[*pos]
}

fn parse_term(s: &Vec<char>, pos: &mut usize) -> (usize, usize) {
    if next_char(s, pos) == '(' {
        read_char(s, pos);
        let result = parse_or(s, pos);
        read_char(s, pos);
        result
    } else {
        read_char(s, pos);
        (1, 1)
    }
}

fn parse_and(s: &Vec<char>, pos: &mut usize) -> (usize, usize) {
    let mut left = parse_term(s, pos);
    while *pos < s.len() && next_char(s, pos) == '&' {
        read_char(s, pos);
        let right = parse_term(s, pos);
        left.0 = cmp::min(left.0, left.1 + right.0);
        left.1 += right.1;
    }
    left
}

fn parse_or(s: &Vec<char>, pos: &mut usize) -> (usize, usize) {
    let mut left = parse_and(s, pos);
    while *pos < s.len() && next_char(s, pos) == '|' {
        read_char(s, pos);
        let right = parse_and(s, pos);
        left.1 = cmp::min(left.1, left.0 + right.1);
        left.0 += right.0;
    }
    left
}

fn parse(s: &Vec<char>) -> (usize, usize) {
    parse_or(s, &mut 0)
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
