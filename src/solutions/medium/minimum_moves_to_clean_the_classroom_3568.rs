/*
 * @lc app=leetcode.cn id=3568 lang=rust
 *
 * [3568] Minimum Moves to Clean the Classroom
 */

pub struct Solution;

// @lc code=start
use std::collections::{HashMap, VecDeque};
struct State {
    x: i16,
    y: i16,
    mask: u16,
    energy: u8,
}

#[derive(PartialEq, Eq)]
enum Thing {
    Litter,
    Reset,
    Obstacle,
    Empty,
}

#[inline]
fn get_pos(x: i16, y: i16, width: i16) -> usize {
    (y * width + x) as usize
}

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let max_energy = energy as u8;
        let (cols, rows) = (classroom[0].len(), classroom.len());
        let size = cols * rows;
        let width = cols as i16;
        let mut queue: VecDeque<State> = VecDeque::new();
        let mut things: Vec<Thing> = Vec::with_capacity(size);
        let mut ls: HashMap<(usize, usize), usize> = HashMap::new();
        let mut all_musk = 0;
        let mut index = 0;
        let (mut sx, mut sy) = (0, 0);
        for (y, row) in classroom.into_iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    'S' => {
                        sx = x as i16;
                        sy = y as i16;
                        things.push(Thing::Empty);
                    }
                    'L' => {
                        ls.insert((x, y), index);
                        all_musk |= 1 << index;
                        things.push(Thing::Litter);
                        index += 1;
                    }
                    'R' => things.push(Thing::Reset),
                    'X' => things.push(Thing::Obstacle),
                    '.' => things.push(Thing::Empty),
                    _ => unreachable!(),
                }
            }
        }

        let mut best: Vec<u8> = vec![0; (all_musk as usize + 1) * size];
        queue.push_back(State {
            x: sx,
            y: sy,
            mask: 0,
            energy: max_energy,
        });

        let d = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        best[get_pos(sx, sy, width)] = max_energy;
        let mut steps = 0;
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let State { x, y, mask, energy } = queue.pop_front().unwrap();
                if mask == all_musk {
                    return steps;
                }
                if energy == 0 {
                    continue;
                }

                let index = mask as usize * size + get_pos(x, y, width);
                if energy < best[index] {
                    continue;
                }

                for (dx, dy) in d {
                    let (x, y, mut mask, mut energy) = (x + dx, y + dy, mask, energy - 1);
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if x >= cols || y >= rows {
                        continue;
                    }
                    let mut maybe_last = false;
                    match things[y * width as usize + x] {
                        Thing::Litter => {
                            let k = ls.get(&(x, y)).unwrap();
                            mask |= 1 << k;
                            if energy == 0 {
                                maybe_last = true
                            };
                        }
                        Thing::Reset => energy = max_energy,
                        Thing::Obstacle => continue,
                        Thing::Empty => {}
                    }
                    let index = mask as usize * size + get_pos(x as i16, y as i16, width);

                    if best[index] < energy || maybe_last {
                        best[index] = energy;
                        queue.push_back(State {
                            x: x as i16,
                            y: y as i16,
                            mask,
                            energy,
                        });
                    }
                }
            }
            steps += 1;
        }

        -1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let room = vec!["LS", "RL"].iter().map(|x| x.to_string()).collect();
        assert_eq!(3, Solution::min_moves(room, 4));
    }

    #[test]
    fn test_2() {
        let room = vec!["L.S", "RXL"].iter().map(|x| x.to_string()).collect();
        assert_eq!(-1, Solution::min_moves(room, 3));
    }

    #[test]
    fn test_3() {
        let room = vec!["S.", "XL"].iter().map(|x| x.to_string()).collect();
        assert_eq!(2, Solution::min_moves(room, 2));
    }

    #[test]
    fn test_4() {
        let room = vec!["...L", "LXRR", "SRX."]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(5, Solution::min_moves(room, 5));
    }

    #[test]
    fn test_5() {
        let room = vec!["LXSLXXX", "XX.R.LL", "R.RLLRR", ".R.LLXR", ".XR..RX"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(-1, Solution::min_moves(room, 32));
    }
}
