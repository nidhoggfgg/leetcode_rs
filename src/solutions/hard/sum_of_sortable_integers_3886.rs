/*
 * @lc app=leetcode.cn id=3886 lang=rust
 *
 * [3886] Sum of Sortable Integers
 */

pub struct Solution;

// @lc code=start
fn frac(n: usize) -> Vec<usize> {
    let mut i = 2;
    let mut result = vec![1, n];
    while i <= n / i {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                result.push(n / i);
            }
        }
        i += 1;
    }
    result
}

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        let fracs = frac(n);
        let mut result = 0;

        let mut un = Vec::with_capacity(n);
        let mut pre_max = Vec::with_capacity(n);
        let mut suffix_min = Vec::with_capacity(n + 1);
        let mut count = 0;
        let mut last = &nums[0];
        let mut max = nums[0];
        for x in &nums {
            if x < last {
                count += 1;
            }
            un.push(count);
            last = x;
            max = max.max(*x);
            pre_max.push(max);
        }

        suffix_min.push(i32::MAX);
        let mut min = i32::MAX;
        for x in nums.iter().rev() {
            min = min.min(*x);
            suffix_min.push(min);
        }
        suffix_min.reverse();

        for k in fracs {
            let mut found = true;
            for i in (0..n).step_by(k) {
                let count = un[i + k - 1] - un[i];
                if count > 1 || (count == 1 && nums[i + k - 1] > nums[i]) {
                    found = false;
                    break;
                }
                if pre_max[i + k - 1] > suffix_min[i + k] {
                    found = false;
                    break;
                }
            }
            if found {
                result += k;
            }
        }
        result as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 1, 2];
        assert_eq!(Solution::sortable_integers(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![423];
        assert_eq!(Solution::sortable_integers(nums), 1);
    }
}
