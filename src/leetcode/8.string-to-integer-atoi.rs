/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let (f, mut s) = s
            .as_bytes()
            .iter()
            .skip_while(|&&c| c == 32)
            .try_fold((None, vec![]), |(mut f, mut s), &c| {
                match c {
                    c if c == 43 || c == 45 => {
                        if f.is_none() && s.is_empty() {
                            f = Some(c)
                        } else {
                            return Err((f, s));
                        }
                    }
                    c if c >= 48 && c <= 57 => s.push(c),
                    _ => return Err((f, s)),
                }
                Ok((f, s))
            })
            .or_else::<(), _>(|e| Ok(e))
            .and_then(|(f, s)| {
                Ok((
                    char::from(f.unwrap_or(43)),
                    String::from_utf8(s.iter().skip_while(|&&c| c == 48).cloned().collect())
                        .unwrap(),
                ))
            })
            .unwrap();
        s.truncate(11);
        s = f.to_string() + &s;
        let mut n: i64 = s.parse().unwrap_or_default();
        if n > std::i32::MAX as _ {
            n = std::i32::MAX as _;
        } else if n < std::i32::MIN as _ {
            n = std::i32::MIN as _;
        }
        n as _
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::my_atoi("42".into()), 42);
    assert_eq!(Solution::my_atoi("   -42".into()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".into()), 4193);
    assert_eq!(Solution::my_atoi("words and 987".into()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".into()), -2147483648);
}
