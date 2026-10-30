use super::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_staircase(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for start in 0..n {
            let mut targets: HashMap<i32, (i32, i32)> = HashMap::new();

            let mut k = 0;
            let mut bad = 0;

            for end in start..n {
                if let Some((c, k)) = targets.get_mut(&nums[end]) {
                    if c != k {
                        bad -= 1;
                    }
                    *c += 1;
                    if c != k {
                        bad += 1;
                    }
                } else {
                    k += 1;
                    targets.insert(nums[end], (1, k));
                    if k != 1 {
                        bad += 1;
                    }
                }
                if bad == 0 {
                    ans += 1;
                }
            }
        }
        ans as i32
    }
}
