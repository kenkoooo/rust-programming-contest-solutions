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

use std::cell::RefCell;
use std::ops::*;
use std::rc::Rc;

const EXP: usize = 1e6 as usize;
const MOD: usize = 1e9 as usize + 7;

fn power(x: ModInt<usize>, e: usize) -> ModInt<usize> {
    let mut result = ModInt::new(1);
    let mut e = e;
    let mut cur = x;
    while e > 0 {
        if e & 1 == 1 {
            result *= cur;
        }
        cur *= cur;
        e >>= 1;
    }
    result
}

#[derive(Copy, Debug)]
pub struct ModInt<T> {
    value: T,
    modulo: T,
}

impl<T> Clone for ModInt<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        ModInt {
            value: self.value,
            modulo: self.modulo,
        }
    }

    fn clone_from(&mut self, source: &ModInt<T>) {
        self.value = source.value;
        self.modulo = source.modulo;
    }
}

impl<T> Add<ModInt<T>> for ModInt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    type Output = ModInt<T>;
    fn add(self, rhs: ModInt<T>) -> ModInt<T> {
        self + rhs.value
    }
}

impl<T> Add<T> for ModInt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    type Output = ModInt<T>;
    fn add(self, rhs: T) -> ModInt<T> {
        let m = self.modulo;
        let mut t = rhs + self.value;
        if t > m {
            t = t - m;
        }
        ModInt {
            value: t,
            modulo: self.modulo,
        }
    }
}

impl<T> Sub<T> for ModInt<T>
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
    type Output = ModInt<T>;
    fn sub(self, rhs: T) -> ModInt<T> {
        let rhs = if rhs >= self.modulo {
            rhs % self.modulo
        } else {
            rhs
        };
        let value = if self.value < rhs {
            self.value + self.modulo
        } else {
            self.value
        };
        ModInt {
            value: value - rhs,
            modulo: self.modulo,
        }
    }
}

impl<T> Sub<ModInt<T>> for ModInt<T>
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
    type Output = ModInt<T>;
    fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
        self - rhs.value
    }
}

impl<T> AddAssign<T> for ModInt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}
impl<T> AddAssign<ModInt<T>> for ModInt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    fn add_assign(&mut self, other: ModInt<T>) {
        *self = *self + other;
    }
}

impl<T> Mul<ModInt<T>> for ModInt<T>
where
    T: Mul<Output = T> + Rem<Output = T> + Copy,
{
    type Output = ModInt<T>;

    fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
        self * rhs.value
    }
}
impl<T> Mul<T> for ModInt<T>
where
    T: Mul<Output = T> + Rem<Output = T> + Copy,
{
    type Output = ModInt<T>;

    fn mul(self, rhs: T) -> ModInt<T> {
        let t = (self.value * rhs) % self.modulo;
        ModInt {
            value: t,
            modulo: self.modulo,
        }
    }
}

impl<T> MulAssign<T> for ModInt<T>
where
    T: Mul<Output = T> + Rem<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> MulAssign<ModInt<T>> for ModInt<T>
where
    T: Mul<Output = T> + Rem<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: ModInt<T>) {
        *self = *self * rhs;
    }
}

impl ModInt<usize> {
    pub fn new(value: usize) -> Self {
        ModInt {
            value: value,
            modulo: MOD,
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    lower: usize,
    upper: usize,
    value: T,
    left: usize,
    right: usize,
}

impl<T> Node<T> {
    fn included(&self, lower: usize, upper: usize) -> bool {
        lower <= self.lower && self.upper <= upper
    }

    fn excluded(&self, lower: usize, upper: usize) -> bool {
        upper <= self.lower || self.upper <= lower
    }

    fn is_single(&self) -> bool {
        self.lower + 1 == self.upper
    }
}

struct SegmentTree<T> {
    mem: Rc<RefCell<Vec<Node<T>>>>,
    zero: T,
    head: usize,
}

impl<T> SegmentTree<T>
where
    T: Copy + Add<Output = T> + std::fmt::Debug,
{
    fn new(n: usize, zero: T, mem_rc: Rc<RefCell<Vec<Node<T>>>>) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let head = mem_rc.borrow().len();
        {
            let mut mem = mem_rc.borrow_mut();

            mem.push(Node {
                lower: 0,
                upper: size,
                value: zero,
                left: 0,
                right: 0,
            });

            let mut pos = head;
            loop {
                let lower = mem[pos].lower;
                let upper = mem[pos].upper;
                if lower + 1 == upper {
                    break;
                }
                let num = upper - lower;
                assert_eq!(num & 1, 0);
                mem[pos].left = mem.len();
                mem[pos].right = mem[pos].left + 1;
                mem.push(Node {
                    lower: lower,
                    upper: lower + num / 2,
                    value: zero,
                    left: 0,
                    right: 0,
                });
                mem.push(Node {
                    lower: lower + num / 2,
                    upper: upper,
                    value: zero,
                    left: 0,
                    right: 0,
                });
                pos += 1;
            }
        }
        SegmentTree {
            mem: mem_rc,
            zero: zero,
            head: head,
        }
    }

    fn update(&mut self, index: usize, value: T) {
        let head = self.head;
        self.update_inner(index, value, head);
    }

    fn update_inner(&mut self, index: usize, value: T, cur: usize) {
        if self.mem.borrow()[cur].included(index, index + 1) {
            self.mem.borrow_mut()[cur].value = value;
        } else if !self.mem.borrow()[cur].excluded(index, index + 1) {
            let left = self.mem.borrow()[cur].left;
            let right = self.mem.borrow()[cur].right;
            self.update_inner(index, value, left);
            self.update_inner(index, value, right);
            self.update_node(cur);
        }
    }

    fn update_node(&mut self, index: usize) {
        let mut mem = self.mem.borrow_mut();
        let left = mem[mem[index].left].value;
        let right = mem[mem[index].right].value;
        mem[index].value = left + right;
    }

    fn query(&self, lower: usize, upper: usize) -> T {
        self.query_inner(lower, upper, self.head)
    }
    fn query_inner(&self, lower: usize, upper: usize, cur: usize) -> T {
        if self.mem.borrow()[cur].included(lower, upper) {
            self.mem.borrow()[cur].value
        } else if self.mem.borrow()[cur].excluded(lower, upper) {
            self.zero
        } else {
            let left = self.mem.borrow()[cur].left;
            let right = self.mem.borrow()[cur].right;
            self.query_inner(lower, upper, left) + self.query_inner(lower, upper, right)
        }
    }

    fn swap(&mut self, other: usize, lower: usize, upper: usize) {
        let cur = self.head;
        self.swap_inner(lower, upper, cur, other);
    }
    fn swap_inner(&mut self, lower: usize, upper: usize, cur: usize, other: usize) {
        if self.mem.borrow()[cur].included(lower, upper) {
            let mut mem = self.mem.borrow_mut();
            if mem[cur].is_single() {
                let cur_value = mem[cur].value;
                mem[cur].value = mem[other].value;
                mem[other].value = cur_value;
            } else {
                let cur_left = mem[cur].left;
                let cur_right = mem[cur].right;
                mem[cur].left = mem[other].left;
                mem[cur].right = mem[other].right;
                mem[other].left = cur_left;
                mem[other].right = cur_right;
            }
        } else if !self.mem.borrow()[cur].excluded(lower, upper) {
            let other_left = self.mem.borrow()[other].left;
            let other_right = self.mem.borrow()[other].right;
            let cur_left = self.mem.borrow()[cur].left;
            let cur_right = self.mem.borrow()[cur].right;

            self.swap_inner(lower, upper, cur_left, other_left);
            self.swap_inner(lower, upper, cur_right, other_right);
        }

        if !self.mem.borrow()[cur].is_single() {
            self.update_node(cur);
            self.update_node(other);
        }
    }
}

// extern crate rand;
// use rand::*;

fn main() {
    input!(
        n: usize,
        m: usize,
        s: [chars; m],
        q: usize,
        queries: [(usize, usize1, usize, usize1, usize1); q]
    );
    solve(n, m, s, q, queries);
    // let mut rng = thread_rng();
    // for i in 0..1000 {
    //     println!("{}", i);
    //     let n = rng.gen::<usize>() % 5 + 1;
    //     let m = rng.gen::<usize>() % 19 + 2;
    //     let s = (0..m).map(|_| (0..n).map(|_| 'a').collect()).collect();
    //     let q = 100000;
    //     let queries = (0..q)
    //         .map(|_| {
    //             let from = rng.gen::<usize>() % n;
    //             let to = rng.gen::<usize>() % n;
    //             let (from, to) = if from > to { (to, from) } else { (from, to) };
    //             (1, 0, 2, from, to)
    //         }).collect();
    //     println!("n={} m={}", n, m);
    //     solve(n, m, s, q, queries);
    // }
}

fn solve(
    n: usize,
    m: usize,
    s: Vec<Vec<char>>,
    q: usize,
    queries: Vec<(usize, usize, usize, usize, usize)>,
) {
    let s: Vec<Vec<usize>> = s
        .iter()
        .map(|s| s.iter().map(|&c| c as usize - 'a' as usize + 1).collect())
        .collect();

    let mem = Rc::new(RefCell::new(Vec::new()));
    let mut segs = vec![];
    for i in 0..m {
        let mut pow = ModInt::new(1);
        let mut seg = SegmentTree::new(n, ModInt::new(0), mem.clone());
        for j in 0..n {
            seg.update(j, pow * s[i][j]);
            pow *= EXP;
        }
        segs.push(seg);
    }

    let mut exp_inv = vec![ModInt::new(1); n + 1];
    exp_inv[1] = power(ModInt::new(EXP), MOD - 2);
    for i in 2..n {
        exp_inv[i] = exp_inv[i - 1] * exp_inv[1];
    }

    for &(t, x, y, l, r) in queries.iter() {
        match t {
            1 => {
                let y = y - 1;
                let other = segs[y].head;
                segs[x].swap(other, l, r + 1);
            }
            2 => {
                let hash = segs[x].query(l, r + 1) * exp_inv[l];
                println!("{}", hash.value);
            }
            _ => unreachable!(),
        }

        // for i in 0..m {
        //     for j in 0..n {
        //         eprint!("{} ", (segs[i].query(j, j + 1) * exp_inv[j]).value);
        //     }
        //     eprintln!();
        // }
        // eprintln!("------------");
    }
}
