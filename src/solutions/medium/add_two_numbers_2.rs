/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use super::Solution;

// @lc code=start
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut now = &mut result;
        let mut l = l1.as_ref();
        let mut r = l2.as_ref();
        let mut plus_one = false;
        loop {
            if l.is_none() && r.is_none() {
                break;
            }
            let lv = if let Some(x) = l {
                l = x.next.as_ref();
                x.val
            } else {
                0
            };
            let rv = if let Some(x) = r {
                r = x.next.as_ref();
                x.val
            } else {
                0
            };
            let mut v = lv + rv;
            if plus_one {
                v += 1;
                plus_one = false;
            }
            if v >= 10 {
                plus_one = true;
                v -= 10;
            }
            let next = ListNode::new(v);
            now.next = Some(Box::new(next));
            now = now.next.as_mut().unwrap();
        }
        if plus_one {
            now.next = Some(Box::new(ListNode::new(1)));
        }
        result.next
    }
}
// @lc code=end
