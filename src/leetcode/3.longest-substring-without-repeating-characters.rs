/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max = 0;
        let chars: Vec<_> = s.chars().collect();

        for end in 0..chars.len() {
            for i in start..end {
                if chars[i] == chars[end] {
                    start = i + 1;
                    break;
                }
            }
            let cur = end - start + 1;
            if cur > max {
                max = cur;
            }
        }

        max as _
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("".into()), 0);
}
