
/*
 * @lc app=leetcode.cn id=3699 lang=rust
 *
 * [3699] Number of ZigZag Arrays I
 */
pub struct Solution;

// @lc code=start
use std::mem::swap;
impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let mm: u64 = 1_000_000_007;
        let m = (r - l + 1) as u64;
        let um = m as usize;
        let mut up = vec![0_u64; um+1];
        let mut down = vec![0_u64; um+1];
        let mut new_down = vec![0_u64; um+1];
        let mut new_up = vec![0_u64; um+1];

        for i in 1..=um {
            up[i] = i as u64 - 1;
            down[i] = m-i as u64;
        }

        for _ in 0..(n-2) {
            let mut prefix = 0;
            for i in 1..=um {
                new_up[i] = prefix;
                prefix += down[i] % mm;
            }

            let mut suffix = 0;
            for i in (1..=um).rev() {
                new_down[i] = suffix;
                suffix += up[i] % mm;
            }

            swap(&mut down, &mut new_down);
            swap(&mut up, &mut new_up);
        }

        let result = (up.iter().sum::<u64>() + down.iter().sum::<u64>()) % mm;

        result as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let l = 4;
        let r = 5;
        assert_eq!(Solution::zig_zag_arrays(n, l, r), 2);
    }

    #[test]
    fn test_2() {
        let n = 3;
        let l = 1;
        let r = 3;
        assert_eq!(Solution::zig_zag_arrays(n, l, r), 10);
    }
}
