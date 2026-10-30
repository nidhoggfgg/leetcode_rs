/*
 * @lc app=leetcode.cn id=155 lang=rust
 *
 * [155] Min Stack
 */

// @lc code=start
struct MinStack {
    stack: Vec<i32>,
    min_index: Option<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_index: None,
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
        if let Some(i) = self.min_index {
            if value < self.stack[i] {
                self.min_index = Some(self.stack.len() - 1);
            }
        } else {
            self.min_index = Some(self.stack.len() - 1);
        }
    }

    fn pop(&mut self) {
        let v = self.stack.pop();
        if v.is_none() {
            return;
        }
        let n = self.stack.len();
        if n == 0 {
            self.min_index = None;
            return;
        }

        let i = self.min_index.unwrap();
        if i >= n {
            let mut min = i32::MAX;
            let mut index = n - 1;
            for j in (0..n).rev() {
                if self.stack[j] <= min {
                    min = self.stack[j];
                    index = j;
                }
            }
            self.min_index = Some(index);
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.stack[self.min_index.unwrap()]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }
}
