/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut result = vec![Vec::<u8>::new(); num_rows];
        let mut i = 0;
        for c in s.as_bytes() {
            if i < num_rows {
                result[i].push(*c);
                i += 1;
            } else if i < num_rows + num_rows - 2 {
                result[num_rows + num_rows - i - 2].push(*c);
                i += 1;
            }
            if i == num_rows + num_rows - 2 {
                i = 0;
            }
        }
        String::from_utf8(result.concat()).unwrap()
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".into(), 3),
        "PAHNAPLSIIGYIR"
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".into(), 4),
        "PINALSIGYAHRPI"
    );
    assert_eq!(Solution::convert("A".into(), 1), "A");
}
