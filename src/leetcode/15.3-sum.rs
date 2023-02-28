/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        for idx in 0..nums.len() {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }
            let target = -nums[idx];
            let mut left = idx + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[left] + nums[right];
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    res.push(vec![nums[idx], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
        return res;
    }
}

// q: is the code wrong?
// a: yes, it is wrong, because it will return the same result multiple times.
// q: can you fix it?
// a: yes, I can fix it, but I don't want to fix it, because I want to learn how to fix it.
// q: i will learn it, as long as you tell me how to fix it.
// a: ok, I will tell you how to fix it.
// code below
// impl Solution {
//     pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut nums = nums;
//         nums.sort();
//         let mut res = vec![];
//         for idx in 0..nums.len() {
//             if idx > 0 && nums[idx] == nums[idx - 1] {
//                 continue;
//             }
//             let target = -nums[idx];
//             let mut left = idx + 1;
//             let mut right = nums.len() - 1;
//             while left < right {
//                 let sum = nums[left] + nums[right];
//                 if sum < target {
//                     left += 1;
//                 }else if sum > target {
//                     right -= 1;
//                 }else {
//                     res.push(vec![nums[idx], nums[left], nums[right]]);
//                     while left < right && nums[left] == nums[left + 1] {
//                         left += 1;
//                     }
//                     while left < right && nums[right] == nums[right - 1] {
//                         right -= 1;
//                     }
//                     left += 1;
//                     right -= 1;
//                 }
//             }
//         }
//         return res
//     }
// }

// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
}
