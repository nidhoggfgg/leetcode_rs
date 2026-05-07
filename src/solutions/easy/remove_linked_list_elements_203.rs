/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 */

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;

        let mut last_node = dummy_head.as_mut();
        // while last_node.next.is_some() {
        //     if last_node.next.as_ref().unwrap().val == val {
        //         last_node.next = last_node.next.as_mut().unwrap().next.take();
        //     } else {
        //         last_node = last_node.next.as_mut().unwrap();
        //     }
        // }

        while let Some(next) = last_node.next.take() {
            if next.val == val {
                last_node.next = next.next;
            } else {
                last_node.next = Some(next);
                last_node = last_node.next.as_mut().unwrap();
            }
        }

        return dummy_head.next;
    }
}
// @lc code=end

