/*
 * @lc app=leetcode.cn id=3970 lang=rust
 *
 * [3970] Shortest Path With At Most K Consecutive Identical Characters
 */

pub struct Solution;

// @lc code=start
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut nexts = vec![vec![]; n];
        for edge in edges {
            nexts[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }
        let cs: Vec<_> = labels.bytes().collect();

        let mut dist = vec![vec![i32::MAX; k + 1]; n];
        let mut pque = BinaryHeap::new();
        dist[0][1] = 0;
        pque.push(Reverse((0, 0, 1)));

        while let Some(Reverse((d, i, count))) = pque.pop() {
            if d > dist[i][count] {
                continue;
            }

            if i == n - 1 {
                return d;
            }

            for &(next, w) in &nexts[i] {
                let v = d + w;
                let c = if cs[i] == cs[next] { count + 1 } else { 1 };
                if c > k {
                    continue;
                }
                if v < dist[next][c] {
                    dist[next][c] = v;
                    pque.push(Reverse((v, next, c)));
                }
            }
        }

        let v = *dist[n - 1].iter().min().unwrap();
        if v == i32::MAX { -1 } else { v }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // n = 3, edges = [[0,1,1],[1,2,1],[0,2,3]], labels = "aab", k = 1
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]];
        let labels = "aab".to_string();
        let k = 1;
        assert_eq!(Solution::shortest_path(n, edges, labels, k), 3);
    }

    #[test]
    fn test_2() {
        //  n = 3, edges = [[0,1,1],[1,2,1],[0,2,3]], labels = "aab", k = 2
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]];
        let labels = "aab".to_string();
        let k = 2;
        assert_eq!(Solution::shortest_path(n, edges, labels, k), 2);
    }

    #[test]
    fn test_3() {
        // n = 3, edges = [[0,1,1],[1,2,1]], labels = "aaa", k = 2
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1]];
        let labels = "aaa".to_string();
        let k = 2;
        assert_eq!(Solution::shortest_path(n, edges, labels, k), -1);
    }
}
