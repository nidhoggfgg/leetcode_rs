/*
 * @lc app=leetcode.cn id=3863 lang=rust
 *
 * [3863] Minimum Operations to Sort a String
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let n = s.len();
        if n == 1 {
            return 0;
        }
        if n == 2 {
            let mut s = s.bytes();
            let a = s.next().unwrap();
            let b = s.next().unwrap();
            if a <= b {
                return 0;
            } else {
                return -1;
            }
        }

        let mut cs = s.bytes();
        let first = cs.next().unwrap();
        let mut last = first;
        let mut max = first;
        let mut min = first;
        let mut sorted = true;
        for x in cs {
            max = max.max(x);
            min = min.min(x);
            sorted = sorted && last <= x;
            last = x;
        }
        if sorted {
            return 0;
        }
        if first <= min || last >= max {
            return 1;
        }
        if last == min && first == max {
            if s.bytes().filter(|x| x == &last || x == &first).count() >= 3 {
                return 2;
            } else {
                return 3;
            }
        }
        2
    }
}
// @lc code=end
