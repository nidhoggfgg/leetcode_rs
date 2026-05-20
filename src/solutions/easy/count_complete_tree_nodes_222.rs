/*
 * @lc app=leetcode.cn id=222 lang=rust
 *
 * [222] Count Complete Tree Nodes
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub struct Solution;

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    // pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if root.is_none() { return 0; }
    //     let root = root.clone().unwrap();
    //     1 + Self::count_nodes(root.borrow().left.clone()) + Self::count_nodes(root.borrow().right.clone())
    // }

    // pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     use std::collections::VecDeque;
    //     let mut count = 0;
    //     if root.is_none() { return count; }
    //     let mut que = VecDeque::new();
    //     que.push_back(root.unwrap());
    //     while !que.is_empty() {
    //         let len = que.len();
    //         count += len as i32;
    //         for _ in 0..len {
    //             let x = que.pop_front().unwrap();
    //             if let Some(n) = x.borrow().left.clone() { que.push_back(n); }
    //             if let Some(n) = x.borrow().right.clone() { que.push_back(n); }
    //         }
    //     }
    //     count
    // }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let root = root.unwrap();
        (match (root.borrow().left.clone(), root.borrow().right.clone()) {
            (Some(left), Some(right)) => {
                let mut ldepth = 1;
                let mut rdepth = 1;
                let mut l= left.clone();
                let mut r = right.clone();
                while let Some(ll) = l.clone().borrow().left.clone() { l = ll; ldepth += 1; }
                while let Some(rr) = r.clone().borrow().right.clone() { r = rr; rdepth += 1; }
                if ldepth == rdepth {
                    2 * (2_i32.pow(ldepth) - 1)
                } else {
                    Self::count_nodes(Some(left)) + Self::count_nodes(Some(right))
                }
            },
            (Some(l), None) => Self::count_nodes(Some(l)),
            (None, Some(r)) => Self::count_nodes(Some(r)),
            (None, None) => 0
        }) + 1
    }
}
// @lc code=end

