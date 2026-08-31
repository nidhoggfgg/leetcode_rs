/*
 * @lc app=leetcode.cn id=2058 lang=rust
 *
 * [2058] Find the Minimum and Maximum Number of Nodes Between Critical Points
 */
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut first = None;
        let mut last = None;
        let mut min_dis = i32::MAX;
        let mut m = head.as_ref().unwrap().val;
        let mut l;
        let mut now = head.as_ref().unwrap().next.as_ref().unwrap().as_ref();
        let mut index = 0;
        while now.next.is_some() {
            let r = now.val;
            (l, m) = (m, r);
            now = now.next.as_ref().unwrap().as_ref();
            let r = now.val;
            index += 1;
            if (l < m && m > r) || (l > m && m < r) {
                if let Some(last_index) = last {
                    min_dis = min_dis.min(index - last_index);
                } else {
                    first = Some(index);
                }
                last = Some(index);
            }
        }

        if last.is_none() || last.unwrap() == first.unwrap() {
            return vec![-1, -1];
        }
        vec![min_dis, last.unwrap() - first.unwrap()]
    }
}
// @lc code=end
