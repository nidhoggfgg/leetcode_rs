/*
 * @lc app=leetcode.cn id=3619 lang=rust
 *
 * [3619] Count Islands With Total Value Divisible by K
 */
pub struct Solution;

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut islands = vec![];
        let mut que = VecDeque::new();
        let (m, n) = (grid[0].len(), grid.len());
        for i in 0..n {
            for j in 0..m {
                let mut value = 0;
                if grid[i][j] != 0 {
                    que.push_back((i, j));
                    value += grid[i][j] as i64;
                    grid[i][j] = 0;
                }
                while que.len() != 0 {
                    let (i, j) = que.pop_front().unwrap();
                    if i + 1 != n && grid[i + 1][j] != 0 {
                        que.push_back((i + 1, j));
                        value += grid[i + 1][j] as i64;
                        grid[i + 1][j] = 0;
                    }
                    if j + 1 != m && grid[i][j + 1] != 0 {
                        que.push_back((i, j + 1));
                        value += grid[i][j + 1] as i64;
                        grid[i][j + 1] = 0;
                    }
                    if i != 0 && grid[i - 1][j] != 0 {
                        que.push_back((i - 1, j));
                        value += grid[i - 1][j] as i64;
                        grid[i - 1][j] = 0;
                    }
                    if j != 0 && grid[i][j - 1] != 0 {
                        que.push_back((i, j - 1));
                        value += grid[i][j - 1] as i64;
                        grid[i][j - 1] = 0;
                    }
                }
                if value != 0 {
                    islands.push(value);
                }
            }
        }
        let mut count = 0;
        let k = k as i64;
        for x in islands {
            if x % k == 0 {
                count += 1;
            }
        }
        count
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let grid = vec![
            vec![0, 2, 1, 0, 0],
            vec![0, 5, 0, 0, 5],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 4, 7, 0],
            vec![0, 2, 0, 0, 8],
        ];
        assert_eq!(Solution::count_islands(grid, 5), 2);
    }
}
