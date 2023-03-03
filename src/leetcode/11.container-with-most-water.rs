/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_ = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        while l < r {
            let w = (r - l) as i32;
            let h = min(height[l], height[r]);
            let area = w * h;
            max_ = max(area, max_);

            if height[l] < height[r] {
                l += 1; // move the short bar inward
                        // because if we move the height bar inward, the new height (min(h[l], h[r]) will be at best the same, at worst smaller)
            } else {
                r -= 1;
            }
        }
        max_
    }
}
// @lc code=end
struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
