/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<_>>();
        let mut start = 0;
        let mut end = 0;
        let mut max = 0;
        for i in 0..chars.len() {
            let mut a = 0;
            loop {
                a += 1;
                if i < a || i + a >= chars.len() {
                    break;
                }
                if chars[i - a] != chars[i + a] {
                    break;
                }
                let cur = 1 + a * 2;
                if cur > max {
                    max = cur;
                    start = i - a;
                    end = i + a;
                }
            }
            a = 0;
            loop {
                a += 1;
                if i < a || i + a - 1 >= chars.len() {
                    break;
                }
                if chars[i - a] != chars[i + a - 1] {
                    break;
                }
                let cur = a * 2;
                if cur > max {
                    max = cur;
                    start = i - a;
                    end = i + a - 1;
                }
            }
        }
        s[start..end + 1].into()
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome("babad".into()), "bab");
    assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
    assert_eq!(Solution::longest_palindrome("a".into()), "a");
    assert_eq!(Solution::longest_palindrome("ac".into()), "a");
}
