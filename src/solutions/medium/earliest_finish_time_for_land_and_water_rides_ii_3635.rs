/*
 * @lc app=leetcode.cn id=3635 lang=rust
 *
 * [3635] Earliest Finish Time for Land and Water Rides II
 */
pub struct Solution;

// @lc code=start
use std::iter::zip;
impl Solution {
    // pub fn earliest_finish_time(
    //     land_start_time: Vec<i32>,
    //     land_duration: Vec<i32>,
    //     water_start_time: Vec<i32>,
    //     water_duration: Vec<i32>,
    // ) -> i32 {
    //     let mut min = i32::MAX;
    //     for (startx, dx) in zip(&land_start_time, &land_duration) {
    //         for (starty, dy) in zip(&water_start_time, &water_duration) {
    //             let land_end = startx + dx;
    //             let water_end = starty + dy;
    //             let land_then_water = if land_end >= *starty {
    //                 land_end + dy
    //             } else {
    //                 starty + dy
    //             };
    //             let water_then_land = if water_end >= *startx {
    //                 water_end + dx
    //             } else {
    //                 startx + dx
    //             };
    //             min = min.min(land_then_water).min(water_then_land);
    //         }
    //     }
    //     min
    // }

    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let mut result = i32::MAX;
        let minland = zip(&land_start_time, &land_duration)
            .map(|(t, d)| t+d).min().unwrap();
        for (t, d) in zip(&water_start_time, &water_duration) {
            result=result.min(if *t>minland{t+d}else{minland+d});
        }

        let minwater = zip(&water_start_time, &water_duration)
            .map(|(t,d)|t+d).min().unwrap();
        for (t,d) in zip(&land_start_time,&land_duration){
            result=result.min(if *t>minwater{t+d}else{minwater+d});
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_then_water_without_waiting() {
        let land_start_time = vec![1];
        let land_duration = vec![3];
        let water_start_time = vec![2];
        let water_duration = vec![2];

        assert_eq!(
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            ),
            6
        );
    }

    #[test]
    fn test_land_then_water_with_waiting() {
        let land_start_time = vec![1];
        let land_duration = vec![2];
        let water_start_time = vec![8];
        let water_duration = vec![3];

        assert_eq!(
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            ),
            11
        );
    }

    #[test]
    fn test_water_then_land_is_better() {
        let land_start_time = vec![10];
        let land_duration = vec![1];
        let water_start_time = vec![1];
        let water_duration = vec![1];

        assert_eq!(
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            ),
            11
        );
    }

    #[test]
    fn test_choose_best_pair_from_multiple_rides() {
        let land_start_time = vec![5, 1, 9];
        let land_duration = vec![4, 7, 2];
        let water_start_time = vec![3, 6, 12];
        let water_duration = vec![5, 2, 1];

        assert_eq!(
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            ),
            10
        );
    }
}
