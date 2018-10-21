/// Thank you tanakh!!!
/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Node {
            left: None,
            right: None,
        }
    }
    fn add(&mut self, s: &[char]) {
        if s.is_empty() {
        } else if s[0] == '0' {
            let x: &mut Box<Node> = self.left.get_or_insert(Box::new(Node::new()));
            x.add(&s[1..]);
        } else {
            let x: &mut Box<Node> = self.right.get_or_insert(Box::new(Node::new()));
            x.add(&s[1..]);
        }
    }

    fn dfs(&self, depth: usize, result: &mut Vec<usize>) {
        if self.left.is_none() != self.right.is_none() {
            result.push(depth);
        }
        if let Some(ref node) = self.left {
            node.dfs(depth + 1, result);
        }
        if let Some(ref node) = self.right {
            node.dfs(depth + 1, result);
        }
    }
}

fn main() {
    input!(n: usize, l: usize, s: [chars; n]);
    let mut trie = Node::new();
    for s in s.iter() {
        trie.add(s);
    }

    let mut result = vec![];
    trie.dfs(0, &mut result);

    let grundy = result
        .iter()
        .map(|&result| {
            let level = l - result;
            let mut cur = 1;
            while level % (cur * 2) == 0 {
                cur *= 2;
            }
            cur
        }).fold(0, |grundy, c| grundy ^ c);
    println!("{}", if grundy == 0 { "Bob" } else { "Alice" });
}
