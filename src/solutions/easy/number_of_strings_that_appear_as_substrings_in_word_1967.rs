/*
 * @lc app=leetcode.cn id=1967 lang=rust
 *
 * [1967] Number of Strings That Appear as Substrings in Word
 */
pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    //     patterns
    //         .iter()
    //         .map(|x| word.contains(x))
    //         .filter(|&x|x)
    //         .count() as i32
    // }

    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        #[derive(Clone)]
        struct State {
            next: Vec<(u8, usize)>,
            link: i32,
            len: usize,
        }

        fn get_next(next: &Vec<(u8, usize)>, c: u8) -> Option<usize> {
            for &(ch, to) in next {
                if ch == c {
                    return Some(to);
                }
            }
            None
        }

        fn set_next(states: &mut Vec<State>, state: usize, c: u8, to: usize) {
            for edge in states[state].next.iter_mut() {
                if edge.0 == c {
                    edge.1 = to;
                    return;
                }
            }
            states[state].next.push((c, to));
        }

        fn extend(states: &mut Vec<State>, last: &mut usize, c: u8) {
            let cur = states.len();

            states.push(State {
                next: Vec::new(),
                link: 0,
                len: states[*last].len + 1,
            });

            let mut p = *last as i32;

            while p != -1 && get_next(&states[p as usize].next, c).is_none() {
                set_next(states, p as usize, c, cur);
                p = states[p as usize].link;
            }

            if p == -1 {
                states[cur].link = 0;
            } else {
                let p_usize = p as usize;
                let q = get_next(&states[p_usize].next, c).unwrap();

                if states[p_usize].len + 1 == states[q].len {
                    states[cur].link = q as i32;
                } else {
                    let clone_next = states[q].next.clone();
                    let clone_link = states[q].link;
                    let clone_len = states[p_usize].len + 1;

                    let clone = states.len();

                    states.push(State {
                        next: clone_next,
                        link: clone_link,
                        len: clone_len,
                    });

                    while p != -1 {
                        let p_usize = p as usize;

                        if get_next(&states[p_usize].next, c) == Some(q) {
                            set_next(states, p_usize, c, clone);
                            p = states[p_usize].link;
                        } else {
                            break;
                        }
                    }

                    states[q].link = clone as i32;
                    states[cur].link = clone as i32;
                }
            }

            *last = cur;
        }

        fn contains(states: &Vec<State>, pattern: &str) -> bool {
            let mut cur = 0usize;

            for &b in pattern.as_bytes() {
                match get_next(&states[cur].next, b) {
                    Some(next) => cur = next,
                    None => return false,
                }
            }

            true
        }

        let mut states = Vec::new();

        states.push(State {
            next: Vec::new(),
            link: -1,
            len: 0,
        });

        let mut last = 0usize;

        for &b in word.as_bytes() {
            extend(&mut states, &mut last, b);
        }

        let mut ans = 0;

        for p in patterns.iter() {
            if contains(&states, p) {
                ans += 1;
            }
        }

        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    fn vstoowned(input: Vec<&str>) -> Vec<String> {
        input.into_iter().map(|x|x.to_owned()).collect()
    }

    #[test]
    fn test_1() {
        let patterns = vec!["a", "abc", "bc", "d"];
        let patterns = vstoowned(patterns);
        let word = "abc".to_owned();
        assert_eq!(Solution::num_of_strings(patterns, word), 3);
    }
}

