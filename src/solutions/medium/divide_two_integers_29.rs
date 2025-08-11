/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] Divide Two Integers
 */


use super::Solution;

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        if dividend == i32::MIN && divisor == 1 {
            return i32::MIN;
        }
        
        let is_negative = (dividend < 0) ^ (divisor < 0);
        
        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        
        if dividend < divisor {
            return 0;
        }
        
        let mut result = 0;
        let mut current_divisor = divisor;
        let mut current_result = 1;
        
        while dividend >= current_divisor {
            if dividend >= current_divisor << 1 {
                current_divisor <<= 1;
                current_result <<= 1;
            } else {
                dividend -= current_divisor;
                result += current_result;
                current_divisor = divisor;
                current_result = 1;
            }
        }
        
        if is_negative {
            -result
        } else {
            result 
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let dividend = 10;
        let divisor = 3;
        let target = 3;
        assert_eq!(Solution::divide(dividend, divisor), target);
    }

    #[test]
    fn test_2() {
        let dividend = 7;
        let divisor = -3;
        let target = -2;
        assert_eq!(Solution::divide(dividend, divisor), target);
    }

    #[test]
    fn test_3() {
        let dividend = 1;
        let divisor = 1;
        let target = 1;
        assert_eq!(Solution::divide(dividend, divisor), target);
    }

    #[test]
    fn test_4() {
        let dividend = -2147483648;
        let divisor = -1;
        let target = 2147483647;
        assert_eq!(Solution::divide(dividend, divisor), target);
    }

    #[test]
    fn test_5() {
        let dividend = -2147483648;
        let divisor = 1;
        let target = -2147483648;
        assert_eq!(Solution::divide(dividend, divisor), target);
    }
}
