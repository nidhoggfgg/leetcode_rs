/*
 * @lc app=leetcode.cn id=3841 lang=rust
 *
 * [3841] Palindromic Path Queries in a Tree
 */
pub struct Solution;

// struct Visited {
//     visited: Vec<u32>,
// }
// impl Visited {
//     fn new(n: usize) -> Self {
//         Self {
//             visited: vec![0; n / 32 + 1],
//         }
//     }
//     fn visited(&self, i: usize) -> bool {
//         let k = i / 32;
//         let i = i % 32;
//         (self.visited[k] & 1 << i) != 0
//     }
//     fn visit(&mut self, i: usize) {
//         let k = i / 32;
//         let i = i % 32;
//         self.visited[k] |= 1 << i;
//     }
//     fn leave(&mut self, i: usize) {
//         let k = i / 32;
//         let i = i % 32;
//         self.visited[k] &= !(1 << i);
//     }
// }
// struct X {
//     chars: Vec<u8>,
//     nexts: Vec<Vec<i32>>,
// }
// impl X {
//     fn dfs(&self, now: usize, end: usize, s: &mut Vec<u8>, visited: &mut Visited) -> bool {
//         s.push(self.chars[now]);
//         if now == end {
//             return true;
//         }
//         visited.visit(now);
//         for x in &self.nexts[now as usize] {
//             let x = *x as usize;
//             if visited.visited(x) {
//                 continue;
//             }
//             if self.dfs(x, end, s, visited) {
//                 return true;
//             }
//         }
//         visited.leave(now);
//         s.pop();
//         false
//     }
//     fn update(&mut self, index: usize, c: u8) {
//         self.chars[index] = c;
//     }
// }
// impl Solution {
//     pub fn palindrome_path(
//         n: i32,
//         edges: Vec<Vec<i32>>,
//         s: String,
//         queries: Vec<String>,
//     ) -> Vec<bool> {
//         let n = n as usize;
//         let mut nexts = vec![vec![]; n];
//         for e in edges {
//             nexts[e[0] as usize].push(e[1]);
//             nexts[e[1] as usize].push(e[0]);
//         }
//         let mut ans = vec![];
//         let chars = s.bytes().collect();
//         let mut x_x = X { chars, nexts };
//         let ua = 'a' as u8;
//         for q in queries {
//             let q: Vec<_> = q.split(' ').collect();
//             let a = q[1].parse().unwrap();
//             if q[0] == "update" {
//                 let b = q[2].bytes().next().unwrap();
//                 x_x.update(a, b);
//             } else if q[0] == "query" {
//                 let b = q[2].parse().unwrap();
//                 let mut s = vec![];
//                 let mut visited = Visited::new(n);
//                 let mut ok = x_x.dfs(a, b, &mut s, &mut visited);
//                 if ok {
//                     let mut counts = [0; 26];
//                     for c in s {
//                         counts[(c - ua) as usize] += 1;
//                     }
//                     ok = counts.iter().filter(|&x| x % 2 != 0).count() <= 1;
//                 }
//                 ans.push(ok);
//             }
//         }
//         ans
//     }
// }

// @lc code=start
impl Solution {
    pub fn palindrome_path(
        _n: i32,
        _edges: Vec<Vec<i32>>,
        _s: String,
        _queries: Vec<String>,
    ) -> Vec<bool> {
        todo!()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let s = "aac".into();
        let queries = vec!["query 0 2", "update 1 b", "query 0 2"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            Solution::palindrome_path(n, edges, s, queries),
            vec![true, false]
        )
    }

    #[test]
    fn test_2() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let s = "abca".into();
        let queries = vec![
            "query 1 2",
            "update 0 b",
            "query 2 3",
            "update 3 a",
            "query 1 3",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(
            Solution::palindrome_path(n, edges, s, queries),
            vec![false, false, true]
        )
    }
}
