/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 */


pub struct Solution;
// Definition for singly-linked list.
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
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;

        let dummy_ptr = &mut *dummy_head as *mut ListNode;
        let mut fast = dummy_ptr as *const ListNode;
        let mut slow = dummy_ptr;

        unsafe {
            for _ in 0..=n {
                // n <= len, unreachable
                if fast.is_null() { return None; }
                fast = match (*fast).next.as_ref() {
                    Some(node) => &**node as *const ListNode,
                    None => std::ptr::null(),
                }
            }
            while !fast.is_null() {
                fast = match (*fast).next.as_ref() {
                    Some(node) => &**node as *const ListNode,
                    None => std::ptr::null(),
                };

                slow = match (*slow).next.as_mut() {
                    Some(node) => &mut **node as *mut ListNode,
                    None => break,
                };
            }

            let mut x = (*slow).next.take().unwrap();
            let y = x.next.take();
            (*slow).next = y;
        }

        dummy_head.next
    }
}
// @lc code=end
