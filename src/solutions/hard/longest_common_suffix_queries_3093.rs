/*
 * @lc app=leetcode.cn id=3093 lang=rust
 *
 * [3093] Longest Common Suffix Queries
 */
pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    //     use std::collections::HashMap;
    //     let mut suffix_map = HashMap::new();
    //     let mut min_len = usize::MAX;
    //     let mut min_len_index = 0;
    //     for (i, x) in words_container.iter().enumerate() {
    //         let len = x.len();
    //         for j in 1..=len {
    //             suffix_map
    //                 .entry(&x[len-j..len])
    //                 .and_modify(|v: &mut (usize, usize)| if v.1>len{v.0=i;v.1=len;})
    //                 .or_insert((i, len));
    //         }
    //         if len < min_len { min_len=len;min_len_index=i; }
    //     }
    //     let min_len_index = min_len_index as i32;
    //     let mut result = Vec::with_capacity(words_query.len());
    //     for x in words_query {
    //         let len = x.len();
    //         let mut found = false;
    //         for i in 0..len {
    //             if let Some((index, _)) = suffix_map.get(&x[i..len]) {
    //                 result.push(index.clone() as i32);
    //                 found = true;
    //                 break;
    //             }
    //         }
    //         if !found { result.push(min_len_index); }
    //     }
    //     result
    // }

    
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        const A: u8 = 'a' as u8;
        struct Node {
            children: [i32; 26],
            best: usize,
            len: usize,
        }
        let mut nodes = vec![Node{children:[-1;26],best:0,len:usize::MAX}];
        for (i, x) in words_container.iter().enumerate() {
            let len = x.len();
            if len<nodes[0].len{nodes[0].len=len;nodes[0].best=i}
            let mut now = 0;
            for b in x.as_bytes().iter().rev() {
                let next = nodes[now].children[(b-A)as usize];
                if next >= 0 {
                    let next = next as usize;
                    if len<nodes[next].len{nodes[next].len=len;nodes[next].best=i;}
                    now=next;
                } else {
                    nodes.push(Node{children:[-1;26],best:i,len});
                    nodes[now].children[(b-A)as usize]=(nodes.len()-1)as i32;
                    now=nodes.len()-1;
                }
            }
        }

        let mut result = Vec::with_capacity(words_query.len());
        for x in words_query {
            let mut now = 0;
            for b in x.as_bytes().iter().rev() {
                let next = nodes[now].children[(b-A)as usize];
                if next>=0 {
                    now=next as usize;
                } else {
                    break;
                }
            }
            result.push(nodes[now].best as i32);
        }

        result
    }
}
// @lc code=end


#[cfg(test)]
mod tests {
use super::Solution;
    fn vstr2vstring(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_1() {
        let input1 = vstr2vstring(vec!["abcd", "bcd", "xbcd"]);
        let input2 = vstr2vstring(vec!["cd", "bcd", "xyz"]);
        let expect = vec![1, 1, 1];
        let result = Solution::string_indices(input1, input2);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_2() {
        let input1 = vstr2vstring(vec!["abcdefgh","poiuygh","ghghgh"]);
        let input2 = vstr2vstring(vec!["gh","acbfgh","acbfegh"]);
        let expect = vec![2, 0, 2];
        let result = Solution::string_indices(input1, input2);
        assert_eq!(expect, result);
    }
}
