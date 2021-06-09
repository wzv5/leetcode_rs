/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut arr = [nums1, nums2].concat();
        arr.sort();
        let i = arr.len() / 2;
        if arr.len() % 2 == 0 {
            (arr[i - 1] + arr[i]) as f64 / 2.0
        } else {
            arr[i] as _
        }
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::find_median_sorted_arrays([1, 3].into(), [2].into()),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays([1, 2].into(), [3, 4].into()),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays([0, 0].into(), [0, 0].into()),
        0.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays([].into(), [1].into()),
        1.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays([2].into(), [].into()),
        2.0
    );
}
