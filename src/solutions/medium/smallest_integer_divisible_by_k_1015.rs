/*
 * @lc app=leetcode.cn id=1015 lang=rust
 *
 * [1015] Smallest Integer Divisible by K
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let bx = match k % 10 {
            1 => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            3 => [0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            7 => [0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            9 => [0, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            _ => unreachable!(),
        };

        // 位数
        let mut n = 0;

        let mut last = 0;
        let mut last_v = 0;
        // a1k + 10a2k + 100a3k + ...
        for _ in 0..100000 {
            // 需要凑出当前位为1
            let an = if last == 0 {
                bx[1]
            } else {
                bx[(11 - last as usize) % 10]
            };
            n += 1;

            // 检查是否全为1
            let v = last_v + an * k;
            let mut vx = v;
            let mut i = 0;
            loop {
                if vx % 10 == 1 {
                    vx = vx / 10;
                    i += 1;
                } else {
                    break;
                }
            }
            if vx == 0 {
                return n + i - 1;
            }

            // 不全为1
            last_v = v / 10;
            last = last_v % 10;
        }

        -1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = Solution::smallest_repunit_div_by_k(1);
        let b = 1;
        assert_eq!(a, b);
    }

    #[test]
    fn test_2() {
        let a = Solution::smallest_repunit_div_by_k(2);
        let b = -1;
        assert_eq!(a, b);
    }

    #[test]
    fn test_3() {
        let a = Solution::smallest_repunit_div_by_k(3);
        let b = 3;
        assert_eq!(a, b);
    }

    #[test]
    fn test_4() {
        let a = Solution::smallest_repunit_div_by_k(111111);
        let b = 6;
        assert_eq!(a, b);
    }

    #[test]
    fn test_5() {
        let a = Solution::smallest_repunit_div_by_k(17);
        let b = 16;
        assert_eq!(a, b);
    }

    #[test]
    fn test_6() {
        let a = Solution::smallest_repunit_div_by_k(149);
        let b = 148;
        assert_eq!(a, b);
    }
}
