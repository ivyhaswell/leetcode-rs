use std::collections::HashMap;

// 最简单的话莫过于暴力法，但时间复杂度达到了O(n^2)；
// 使用哈希表缓存之前的值和index，可以将时间复杂度降低到O(n)；
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            if let Some(pre_index) = map.get(&(target - nums[i])) {
                return vec![*pre_index, i as i32];
            } else {
                map.insert(nums[i], i as i32);
            }
        }

        vec![]
    }
}

struct Solution {}

#[test]
pub fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
