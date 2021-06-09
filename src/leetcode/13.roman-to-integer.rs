/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: std::collections::HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();
        let chars: Vec<_> = s.chars().collect();
        let mut result = map[&chars[0]];
        for i in 1..chars.len() {
            let v = map.get(&chars[i]).unwrap();
            let v0 = map.get(&chars[i - 1]).unwrap();
            if v > v0 {
                result += v - 2 * v0;
            } else {
                result += v;
            }
        }
        result
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::roman_to_int("III".into()), 3);
    assert_eq!(Solution::roman_to_int("IV".into()), 4);
    assert_eq!(Solution::roman_to_int("IX".into()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
}
