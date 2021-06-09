/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        [[-1, -1, 2], [-1, 0, 1]]
    );
    assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<i32>>::new());
}
