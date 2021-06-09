/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let f = x < 0;
        let s = String::from_utf8(
            x.abs()
                .to_string()
                .as_bytes()
                .iter()
                .rev()
                .cloned()
                .collect(),
        )
        .unwrap();
        let mut a: i32 = s.parse().unwrap_or_default();
        if f {
            a = -a;
        }
        a
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
}
