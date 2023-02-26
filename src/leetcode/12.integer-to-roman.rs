/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        // for the input num is between 1 and 3999, there will be four digits at most.
        let thousands = ["", "M", "MM", "MMM"];
        let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        thousands[(num / 1000) as usize].to_string()
            + hundreds[(num % 1000 / 100) as usize]
            + tens[(num % 100 / 10) as usize]
            + ones[(num % 10) as usize]
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
    assert_eq!(Solution::int_to_roman(997), "CMXCVII".to_string());
}
