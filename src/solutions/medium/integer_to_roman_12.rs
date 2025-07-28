/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] Integer to Roman
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num as usize;
        let mut result = String::new();
        let symbols = [
            (1000, 900, "M", "CM"),
            (500, 400, "D", "CD"),
            (100, 90, "C", "XC"),
            (50, 40, "L", "XL"),
            (10, 9, "X", "IX"),
            (5, 4, "V", "IV"),
            (1, 0, "I", ""),
        ];

        for (v1, v2, sym, sub_sym) in symbols {
            if num < v2 {
                continue;
            }
            result.push_str(&sym.repeat(num / v1));
            num %= v1;
            if num >= v2 {
                num -= v2;
                result.push_str(sub_sym);
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::medium::integer_to_roman_12::Solution;

    #[test]
    fn test_1() {
        let num = 3;
        assert_eq!(Solution::int_to_roman(num), "III")
    }

    #[test]
    fn test_2() {
        let num = 3749;
        assert_eq!(Solution::int_to_roman(num), "MMMDCCXLIX")
    }
}
