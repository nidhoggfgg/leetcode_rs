/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue {
    instack: Vec<i32>,
    outstack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            instack: vec![],
            outstack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.instack.push(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(v) = self.outstack.pop() {
            return v;
        } else {
            self.intoout();
            return self.outstack.pop().unwrap();
        }
    }

    fn peek(&mut self) -> i32 {
        if let Some(v) = self.outstack.last() {
            return *v;
        } else {
            self.intoout();
            return *self.outstack.last().unwrap();
        }
    }

    fn empty(&self) -> bool {
        self.instack.is_empty() && self.outstack.is_empty()
    }

    fn intoout(&mut self) {
        let mut instack = std::mem::take(&mut self.instack);
        while let Some(v) = instack.pop() {
            self.outstack.push(v);
        }
    }
}

// @lc code=end
