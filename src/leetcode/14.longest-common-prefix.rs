/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter()
            .skip(1)
            .fold(strs[0].as_str(), |a, s| {
                let minstr = if a.len() > s.len() { s.as_str() } else { a };
                for i in 0..minstr.len() {
                    if a.as_bytes()[i] != s.as_bytes()[i] {
                        return &a[..i];
                    }
                }
                minstr
            })
            .to_string()
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::longest_common_prefix(
            ["flower", "flow", "flight"]
                .iter()
                .map(|i| i.to_string())
                .collect()
        ),
        "fl"
    );
    assert_eq!(
        Solution::longest_common_prefix(
            ["dog", "racecar", "car"]
                .iter()
                .map(|i| i.to_string())
                .collect()
        ),
        ""
    );
}
