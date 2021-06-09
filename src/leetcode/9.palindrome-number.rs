/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }
        let mut a = 0;
        while x > a {
            a = a * 10 + x % 10;
            x /= 10;
        }
        return x == a || x == a / 10;
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(-101), false);
    assert_eq!(Solution::is_palindrome(1221), true);
}
