use std::cmp;
use std::i64::MIN;

fn main() {
    let (h, w) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };

    let p = {
        let mut p = vec![vec![0; w]; h + 1];
        for i in 0..h {
            let v = read_values::<i64>();
            for j in 0..w {
                p[i][j] = v[j];
            }
        }
        p
    };

    let f = {
        let mut f = vec![vec![0; w]; h + 1];
        for i in 0..h {
            let v = read_values::<i64>();
            for j in 0..w {
                f[i][j] = v[j];
            }
        }
        f
    };

    let mut gain = vec![vec![MIN; w]; h + 1];
    gain[0][0] = p[0][0];

    for i in 0..h {
        let mut left_turn = vec![0; w];
        for j in 1..w {
            left_turn[j] = cmp::max(left_turn[j - 1] + p[i][j - 1] - (f[i][j - 1] + f[i][j]), 0);
        }
        let mut right_turn = vec![0; w];
        for j in (0..(w - 1)).rev() {
            right_turn[j] = cmp::max(right_turn[j + 1] + p[i][j + 1] - (f[i][j + 1] + f[i][j]), 0);
        }

        let mut sum = vec![0; w + 1];
        for j in 0..w {
            sum[j + 1] = sum[j] + p[i][j] - f[i][j];
        }

        if i == 0 {
            for j in 0..w {
                gain[i + 1][j] = p[i + 1][j] - f[i + 1][j] + sum[j + 1] + right_turn[j];
            }
        } else {
            let mut left_max = 0;
            for j in 0..w {
                let segment_sum = sum[j + 1] - sum[left_max + 1];
                let enter_gain = p[i + 1][j] - f[i + 1][j];

                let old_gain = gain[i][left_max] + segment_sum + left_turn[left_max];
                let new_gain = gain[i][j] + left_turn[j];
                if old_gain < new_gain {
                    left_max = j;
                }

                gain[i + 1][j] = cmp::max(old_gain, new_gain) + right_turn[j] + enter_gain;
            }

            let mut right_max = w - 1;
            for j in (0..w).rev() {
                let segment_sum = sum[right_max] - sum[j];
                let enter_gain = p[i + 1][j] - f[i + 1][j];

                let old_gain = gain[i][right_max] + segment_sum + right_turn[right_max];
                let new_gain = gain[i][j] + right_turn[j];
                if old_gain < new_gain {
                    right_max = j;
                }

                gain[i + 1][j] = cmp::max(
                    cmp::max(old_gain, new_gain) + left_turn[j] + enter_gain,
                    gain[i + 1][j],
                );
            }
        }
    }
    for i in 0..w {
        println!("{}", gain[h][i]);
    }
}


fn read_line() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
{
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}