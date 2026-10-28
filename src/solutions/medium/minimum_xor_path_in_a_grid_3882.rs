/*
 * @lc app=leetcode.cn id=3882 lang=rust
 *
 * [3882] Minimum XOR Path in a Grid
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid[0].len(), grid.len());
        let mut last_line: Vec<Vec<usize>> = vec![vec![]; m];
        last_line[0].push(grid[0][0] as usize);
        let mut last = grid[0][0] as usize;
        for j in 1..m {
            let v = last ^ grid[0][j] as usize;
            last_line[j].push(v);
            last = v;
        }

        let mut seen = [0u16; 1024];
        let mut stamp = 1;
        for i in 1..n {
            let g = grid[i][0] as usize;
            last_line[0].iter_mut().for_each(|x| *x = *x ^ g);

            for j in 1..m {
                let mut current = std::mem::take(&mut last_line[j]);
                let other = &last_line[j - 1];

                for x in &current {
                    seen[*x] = stamp;
                }
                for x in other {
                    if seen[*x] != stamp {
                        current.push(*x);
                        seen[*x] = stamp;
                    }
                }

                if current.len() == 1024 {
                    return 0;
                }

                let g = grid[i][j] as usize;
                current.iter_mut().for_each(|x| *x = *x ^ g);
                last_line[j] = current;
                stamp += 1;
            }
        }

        *last_line.last().unwrap().iter().min().unwrap() as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![3, 4]]), 6);
    }
}
