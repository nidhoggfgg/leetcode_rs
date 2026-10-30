use super::Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut k = k;
        let mut diff = Vec::with_capacity(n);
        let mut ans: i64 = nums.iter().map(|x| *x as i64).sum();
        for x in nums {
            let mut v = x;
            let mut counts = [0; 10];
            while v > 0 {
                counts[v as usize % 10] += 1;
                v /= 10;
            }

            let mut t = 0;
            for i in (0..10).rev() {
                while counts[i] > 0 {
                    t *= 10;
                    t += i as i64;
                    counts[i] -= 1;
                }
            }
            diff.push(t - x as i64);
        }

        diff.sort_unstable();
        for d in diff.into_iter().rev() {
            if k == 0 {
                break;
            }
            if d > 0 {
                ans += d as i64;
                k -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums = vec![123, 45];
        let k = 1;
        assert_eq!(Solution::maximize_sum(nums, k), 366);
    }
}
