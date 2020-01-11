use mod_int::ModInt;
const MOD: usize = 1e9 as usize + 7;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut sizes = vec![0; n];
    let mut parent = vec![0; n];
    calc_subtree_size(0, 0, &graph, &mut sizes, &mut parent);
    //    eprintln!("{:?}", sizes);

    let mut sum = ModInt(0);
    for v in 0..n {
        if graph[v].len() == 1 {
            continue;
        }
        let mut ans = vec![];
        for &next in graph[v].iter() {
            if next == parent[v] {
                let size = n - sizes[v];
                let no_black = (ModInt(1) / ModInt(2)).pow(size);
                ans.push(no_black);
            } else {
                let no_black = (ModInt(1) / ModInt(2)).pow(sizes[next]);
                ans.push(no_black);
            }
        }

        //        eprintln!("v={} ans={:?}", v, ans);

        let mut all_white = ModInt(1);
        for &white_p in ans.iter() {
            all_white *= white_p;
        }

        let mut no_2_black = all_white;
        for &p in ans.iter() {
            let one_black = all_white / p * (ModInt(1) - p);
            no_2_black += one_black;
        }

        let two_or_more_black = ModInt(1) - no_2_black;
        sum += two_or_more_black / 2;
        //        eprintln!("v={} p={}", v, two_or_more_black.0);
        //        eprintln!(
        //            "all_white={:?} no_2_black={:?} two_blaok={:?}",
        //            all_white, no_2_black, two_or_more_black
        //        );
    }

    println!("{}", sum.0);
}

fn calc_subtree_size(
    v: usize,
    parent: usize,
    graph: &Vec<Vec<usize>>,
    sizes: &mut Vec<usize>,
    p: &mut Vec<usize>,
) -> usize {
    p[v] = parent;
    for &next in graph[v].iter() {
        if next == parent {
            continue;
        }
        sizes[v] += calc_subtree_size(next, v, graph, sizes, p);
    }
    sizes[v] += 1;
    sizes[v]
}

pub mod mod_int {
    use super::MOD;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;

    #[derive(Clone, Copy, Debug)]
    pub struct ModInt<T: Copy + Clone>(pub T);

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.0
        }
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: Num) -> ModInt<Num> {
            let mut t = rhs + self.0;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt(t)
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.0 < rhs { self.0 + MOD } else { self.0 };
            ModInt(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.0
        }
    }

    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other;
        }
    }
    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other;
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other;
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other;
        }
    }

    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: Num) -> ModInt<Num> {
            self * ModInt(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.0
        }
    }

    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }
    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;

        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.0
        }
    }
    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;

        fn mul(self, rhs: Num) -> ModInt<Num> {
            let t = (self.0 * rhs) % MOD;
            ModInt(t)
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs;
        }
    }

    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs;
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: usize) -> ModInt<Num> {
            let mut result = ModInt(1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
