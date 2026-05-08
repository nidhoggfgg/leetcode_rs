/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] Reverse Linked List
 */

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// @lc code=start
// Definition for singly-linked list.
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut next  = head;
        while let Some(mut curr) = next {
            next = curr.next;
            curr.next = pre.take();
            pre = Some(curr);
        }
        pre
    }
}
// @lc code=end

