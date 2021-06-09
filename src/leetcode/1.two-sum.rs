/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let a = target - v;
            if map.contains_key(&a) {
                return vec![map[&a], i as i32];
            }
            map.insert(v, i as i32);
        }
        unreachable!();
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::two_sum([2, 7, 11, 15].into(), 9), [0, 1]);
    assert_eq!(Solution::two_sum([3, 2, 4].into(), 6), [1, 2]);
    assert_eq!(Solution::two_sum([3, 3].into(), 6), [0, 1]);
}
