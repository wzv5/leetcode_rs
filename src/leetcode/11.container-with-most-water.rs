/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut h1 = 0;
        let mut h2 = height.len() - 1;
        while h1 < h2 {
            let h = h2 - h1;
            let cur = std::cmp::min(height[h1], height[h2]) * h as i32;
            if cur > max {
                max = cur;
            }
            if height[h1] > height[h2] {
                h2 -= 1;
            } else {
                h1 += 1;
            }
        }
        max
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::max_area([1, 8, 6, 2, 5, 4, 8, 3, 7].into()), 49);
    assert_eq!(Solution::max_area([1, 1].into()), 1);
    assert_eq!(Solution::max_area([4, 3, 2, 1, 4].into()), 16);
    assert_eq!(Solution::max_area([1, 2, 1].into()), 2);
}
