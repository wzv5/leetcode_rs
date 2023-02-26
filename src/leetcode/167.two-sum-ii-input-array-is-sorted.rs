/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input Array Is Sorted
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i < j {
            let sum = numbers[i] + numbers[j];
            if target == sum {
                return vec![(i + 1) as i32, (j + 1) as i32];
            }

            if sum > target {
                j -= 1;
            }
            if sum < target {
                i +=1;
            }
        }
        unreachable!()
    }
}
// @lc code=end
struct Solution;

mod tests{
    use super::*;
    #[test]
    fn test_two_sum_ii() {
        assert_eq!(Solution::two_sum([2, 7, 11, 15].into(), 9), [1, 2]);
        assert_eq!(Solution::two_sum([2, 3, 4].into(), 6), [1, 3]);
    }
}