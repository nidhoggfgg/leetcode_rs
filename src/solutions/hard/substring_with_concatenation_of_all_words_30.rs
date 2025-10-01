/*
 * @lc app=leetcode.cn id=30 lang=rust
 *
 * [30] Substring with Concatenation of All Words
 */


use super::Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // if words.is_empty() || s.is_empty() {
        //     return vec![];
        // }
        let word_len = words[0].len();
        let word_count = words.iter().fold(HashMap::new(), |mut acc, word| {
            if let Some(v) = acc.get_mut(word) {
                *v += 1;
            } else {
                acc.insert(word.clone(), 1);
            }
            acc
        });
        let total_len = word_len * words.len();
        let n = s.len();

        if n < total_len {
            return vec![];
        }

        let s_bytes = s.as_bytes();
        let mut result = Vec::new();

        for start in 0..word_len {
            let mut left = start;
            let mut right = start;
            let mut window_count: HashMap<String, i32> = HashMap::new();
            let mut formed = 0;

            while right + word_len <= n {
                let word = std::str::from_utf8(&s_bytes[right..right + word_len])
                    .unwrap()
                    .to_string();
                right += word_len;

                if word_count.contains_key(&word) {
                    if let Some(v) = window_count.get_mut(&word) {
                        *v += 1;
                    } else {
                        window_count.insert(word.clone(), 1);
                    }

                    if window_count[&word] == word_count[&word] {
                        formed += 1;
                    }
                } else {
                    window_count.clear();
                    formed = 0;
                    left = right;
                    continue;
                }

                while right - left > total_len {
                    let left_word = std::str::from_utf8(&s_bytes[left..left + word_len])
                        .unwrap()
                        .to_string();
                    left += word_len;

                    if word_count.contains_key(&left_word) {
                        if window_count[&left_word] == word_count[&left_word] {
                            formed -= 1;
                        }
                        *window_count.get_mut(&left_word).unwrap() -= 1;
                    }
                }

                if right - left == total_len && formed == word_count.len() {
                    result.push(left as i32);
                }
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo", "bar"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::find_substring(s, words), [0, 9]);
    }

    #[test]
    fn test_2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec!["word", "good", "best", "word"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::find_substring(s, words), []);
    }

    #[test]
    fn test_3() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec!["word", "good", "best", "good"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::find_substring(s, words), [8]);
    }

    #[test]
    fn test_4() {
        let s = "gggx".to_string();
        let words = vec!["g", "g", "x"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::find_substring(s, words), [1]);
    }
}

mod attempt {
    use super::Solution;
    use std::collections::{HashMap, HashSet, VecDeque};

    impl Solution {
        pub fn find_substring_impl(mut s: String, words: Vec<String>) -> Vec<i32> {
            struct R {
                start: usize,           // 最开始的位置
                now: usize,             // 当前在判断哪个words
                index: usize,           // words的第几个字符
                append: HashSet<usize>, // 已经加入的words
            }

            let mut chars_map: Vec<Vec<u8>> = Vec::with_capacity(words.len());
            let mut first_indexes: HashMap<u8, Vec<usize>> = HashMap::new();
            let wl = words[0].len();
            let words_len = words.len();
            for (i, w) in words.iter().enumerate() {
                let first_char = w.as_bytes()[0];
                if let Some(l) = first_indexes.get_mut(&first_char) {
                    l.push(i);
                } else {
                    first_indexes.insert(first_char, vec![i]);
                }
                chars_map.push(w.as_bytes().to_owned());
            }

            s.push('\0');
            let mut real_results = HashSet::new();
            let mut results: VecDeque<R> = VecDeque::new();
            for (i, c) in s.as_bytes().iter().enumerate() {
                let len = results.len();
                for _ in 0..len {
                    let mut r = results.pop_front().unwrap();
                    if r.index == wl {
                        if r.append.len() == words_len - 1 {
                            real_results.insert(r.start as i32);
                            continue;
                        }
                        if let Some(l) = first_indexes.get(c) {
                            for x in l {
                                if r.now == *x || r.append.get(x) != None {
                                    continue;
                                }
                                let mut append = r.append.clone();
                                append.insert(r.now);
                                results.push_back(R {
                                    start: r.start,
                                    now: *x,
                                    index: 1,
                                    append: append,
                                });
                            }
                        }
                        continue;
                    }
                    if chars_map[r.now][r.index] == *c {
                        r.index += 1;
                        results.push_back(r);
                    }
                }

                if let Some(l) = first_indexes.get(c) {
                    for x in l {
                        results.push_back(R {
                            start: i,
                            now: *x,
                            index: 1,
                            append: HashSet::new(),
                        });
                    }
                }
            }

            real_results.into_iter().collect()
        }
    }
}
