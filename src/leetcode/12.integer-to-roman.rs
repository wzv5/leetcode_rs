/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let num = num as usize;
        let m = ["", "M", "MM", "MMM"];
        let c = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let x = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let i = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        format!(
            "{}{}{}{}",
            m[num / 1000],
            c[(num % 1000) / 100],
            x[(num % 100) / 10],
            i[num % 10]
        )
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::int_to_roman(3), "III");
    assert_eq!(Solution::int_to_roman(4), "IV");
    assert_eq!(Solution::int_to_roman(9), "IX");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}
