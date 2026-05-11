/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        let mut pre = &mut dummy_head;
        loop {
            let mut cur = pre.as_mut().next.take();
            if cur.is_none() {
                pre.next = cur;
                break
            }
            let mut next = cur.as_mut().unwrap().next.take();
            if next.is_none() {
                cur.as_mut().unwrap().next = next;
                pre.next = cur;
                break
            }

            cur.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = cur;
            pre.as_mut().next = next;
            pre = pre.as_mut().next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy_head.next
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> bool {
        loop {
            match (&a, &b) {
                (Some(x), Some(y)) => if x.val != y.val {return false},
                (None, None) => return true,
                (_, _) => return false
            }
            a = a.unwrap().next;
            b = b.unwrap().next;
        }
    }

    #[test]
    fn test_1() {
        let a = Solution::swap_pairs(Some(Box::new(ListNode::new(1))));
        let b = Some(Box::new(ListNode::new(1)));
        assert!(compare(a, b));
    }
}