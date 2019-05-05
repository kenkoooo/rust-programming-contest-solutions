/// Thank you tanakh!!!
///  https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::VecDeque;

trait PeekDeque<T> {
    fn head(&mut self) -> T;
}

impl PeekDeque<char> for VecDeque<char> {
    fn head(&mut self) -> char {
        *self.iter().next().unwrap()
    }
}

#[derive(Debug)]
enum Element {
    Single(String),
    Expr(Vec<Element>, Sign),
}

#[derive(Debug)]
enum Sign {
    Add,
    Sub,
    Mul,
    Div,
    Empty,
}

fn parse_num(q: &mut VecDeque<char>) -> String {
    let mut result = String::new();
    while !q.is_empty() && q.head().is_digit(10) {
        let s = q.pop_front().unwrap();
        result.push(s);
    }
    result
}

fn parse_element(q: &mut VecDeque<char>) -> Element {
    let sign = match q.head() {
        '+' => Sign::Add,
        '-' => Sign::Sub,
        '*' => Sign::Mul,
        '/' => Sign::Div,
        _ => Sign::Empty,
    };

    match sign {
        Sign::Empty => {
            let num = parse_num(q);
            Element::Single(num)
        }
        _ => {
            q.pop_front();
            let c = q.pop_front().unwrap();
            assert_eq!(c, '(');

            let mut result = vec![];
            loop {
                let e = parse_element(q);
                result.push(e);
                if q.head() == ',' {
                    q.pop_front();
                } else {
                    let c = q.pop_front().unwrap();
                    assert_eq!(c, ')');
                    break;
                }
            }
            Element::Expr(result, sign)
        }
    }
}

fn print_element(s: &mut String, element: &Element) {
    match element {
        Element::Single(ref t) => s.push_str(t),
        Element::Expr(elements, sign) => {
            let sign = match sign {
                Sign::Add => '+',
                Sign::Sub => '-',
                Sign::Mul => '*',
                Sign::Div => '/',
                _ => unreachable!(),
            };
            s.push('(');
            for (i, e) in elements.iter().enumerate() {
                if i > 0 {
                    s.push(sign);
                }
                print_element(s, e);
            }
            s.push(')');
        }
    }
}

fn main() {
    input!(s: chars);
    let mut q: VecDeque<char> = s.iter().map(|&c| c).collect();
    let e = parse_element(&mut q);

    let mut ans = String::new();
    print_element(&mut ans, &e);
    println!("{}", ans);
}
