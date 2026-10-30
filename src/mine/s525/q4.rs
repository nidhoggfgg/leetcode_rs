pub struct Solution;

impl Solution {
    pub fn count_staircase(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_k = 0;
        while (max_k + 1) * (max_k + 2) / 2 <= n {
            max_k += 1;
        }

        let mut seen = vec![0usize; 100001];
        let mut count = vec![0usize; 100001];
        let mut rank = vec![0usize; 100001];
        let mut ans = 0;
        for start in 0..n {
            let stamp = start + 1;
            let mut k = 0;
            for end in start..n {
                let x = nums[end] as usize;
                if seen[x] != stamp {
                    k += 1;
                    if k > max_k {
                        break;
                    }
                    seen[x] = stamp;
                    count[x] = 0;
                    rank[x] = k;
                }
                count[x] += 1;
                if count[x] > rank[x] {
                    break;
                }
                let len = end - start + 1;
                if len == k * (k + 1) / 2 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
