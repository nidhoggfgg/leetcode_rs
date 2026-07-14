/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// @lc code=start
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        while list1.is_some() && list2.is_some() {
            let source=if list1.as_ref().unwrap().val>list2.as_ref().unwrap().val{
                &mut list2
            } else {
                &mut list1
            };
            let mut node=source.take().unwrap();
            *source=node.next.take();
            let x =tail.insert(node);
            tail=&mut x.next;
        }
        *tail=list1.or(list2);
        head
    }
}
// @lc code=end

