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

use std::collections::BinaryHeap;

const INF: usize = 1e15 as usize;
const MAX_D: usize = 1e7 as usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    i: usize,
    j: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input!(h: usize, w: usize, k: usize, d: chars, s: [chars; h]);

    let mut dir = vec![vec![]; 4];

    for (i, &d) in d.iter().enumerate() {
        match d {
            'U' => dir[0].push(i),
            'D' => dir[1].push(i),
            'L' => dir[2].push(i),
            'R' => dir[3].push(i),
            _ => unreachable!(),
        }
    }

    let d = vec!['U', 'D', 'L', 'R'];

    let mut queue = BinaryHeap::new();
    let mut dist = vec![vec![INF; w]; h];
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match s[i][j] {
                'S' => {
                    queue.push(State {
                        i: i,
                        j: j,
                        cost: 0,
                    });
                    dist[i][j] = 0;
                }
                'G' => {
                    goal = (i, j);
                }
                _ => {}
            }
        }
    }

    while let Some(State { i, j, cost }) = queue.pop() {
        if i == goal.0 && j == goal.1 {
            println!("{}", cost);
            return;
        }
        for tmp in 0..4 {
            if dir[tmp].is_empty() {
                continue;
            }
            let key = dir[tmp]
                .binary_search_by_key(&((cost % k) * 2 + 1), |&i| i * 2 + 2)
                .err()
                .unwrap();
            let next_t = if key == dir[tmp].len() {
                dir[tmp][0] + 1
            } else {
                dir[tmp][key] + 1
            };
            let next_t = if next_t <= cost {
                let mut t = cost / k * k + next_t;
                while t <= cost {
                    t += k;
                }
                t
            } else {
                next_t
            };

            match d[tmp] {
                'U' => if i > 0 && dist[i - 1][j] > next_t && s[i - 1][j] != '#' {
                    queue.push(State {
                        i: i - 1,
                        j: j,
                        cost: next_t,
                    });
                    dist[i - 1][j] = next_t;
                },
                'D' => if i + 1 < h && dist[i + 1][j] > next_t && s[i + 1][j] != '#' {
                    queue.push(State {
                        i: i + 1,
                        j: j,
                        cost: next_t,
                    });
                    dist[i + 1][j] = next_t;
                },
                'L' => {
                    if j > 0 && dist[i][j - 1] > next_t && s[i][j - 1] != '#' {
                        queue.push(State {
                            i: i,
                            j: j - 1,
                            cost: next_t,
                        });
                        dist[i][j - 1] = next_t;
                    }
                }
                'R' => {
                    if j + 1 < w && dist[i][j + 1] > next_t && s[i][j + 1] != '#' {
                        queue.push(State {
                            i: i,
                            j: j + 1,
                            cost: next_t,
                        });
                        dist[i][j + 1] = next_t;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    println!("-1");
}
