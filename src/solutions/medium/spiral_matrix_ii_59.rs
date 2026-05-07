/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */


pub struct Solution;

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        // let mut result = vec![vec![0;n as usize]; n as usize];
        // let (mut i, mut j): (i32, i32) = (0, 0);
        // let (mut si, mut sj) = (0, 1);
        // for x in 1..=n*n {
        //     result[i as usize][j as usize] = x;
        //     if i + j == n - 1 {
        //         if i <= n/2 {
        //             (si, sj) = (1, 0);
        //         } else {
        //             (si, sj) = (-1, 0);
        //         }
        //     }
        //     if i == j + 1 && i <= n/2 {
        //         (si, sj) = (0, 1);
        //     }
        //     if i == j && i >= n/2 {
        //         (si, sj) = (0, -1);
        //     }
        //     i += si;
        //     j += sj;
        // }
        // result

        let n = n as usize;
        let mut p = 0;
        let mut result = vec![vec![0_i32; n as usize]; n as usize];
        let mut start = 1;
        while p < n/2 {
            let mut now = start;
            let l = n - 2*p - 1;
            for i in 0..l {
                result[p][p+i] = now;
                result[p+i][n-p-1] = now + l as i32;
                result[n-p-1][n-p-i-1] = now + 2 * l as i32;
                result[n-p-i-1][p] = now + 3 * l as i32;
                now += 1;
            }

            start += 4*l as i32;
            p += 1;
        }

        if n % 2 != 0 {
            result[n/2][n/2] = (n * n) as i32;
        }

        result
    }
}
// @lc code=end

