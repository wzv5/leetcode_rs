/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }
        let mut a = 0;
        // reverse the number
        while x > a {
            a = a * 10 + x % 10;
            x /= 10;
        }
        x == a || x == a / 10
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(0), true);
    assert_eq!(Solution::is_palindrome(1), true);
    assert_eq!(Solution::is_palindrome(11), true);
    assert_eq!(Solution::is_palindrome(111), true);
    assert_eq!(Solution::is_palindrome(1111), true);
}