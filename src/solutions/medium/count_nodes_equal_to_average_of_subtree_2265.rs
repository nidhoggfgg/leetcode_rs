/*
 * @lc app=leetcode.cn id=2265 lang=rust
 *
 * [2265] Count Nodes Equal to Average of Subtree
 */

pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(unused)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = Solver { count: 0 };
        if let Some(root) = root {
            s.travese(&root.as_ref().borrow());
            return s.count;
        }
        -1
    }
}

struct Solver {
    count: i32,
}

impl Solver {
    fn travese(&mut self, node: &TreeNode) -> (i32, i32) {
        let mut sum = node.val;
        let mut c = 1;
        if let Some(l) = node.left.as_ref() {
            let (x, n) = self.travese(&l.borrow());
            sum += x;
            c += n;
        }
        if let Some(r) = node.right.as_ref() {
            let (x, n) = self.travese(&r.borrow());
            sum += x;
            c += n;
        }
        if sum / c == node.val {
            self.count += 1;
        }

        (sum, c)
    }
}
// @lc code=end
