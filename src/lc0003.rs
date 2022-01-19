use std::collections::HashSet;

impl Solution {
    // 滑动窗口，用一个HashSet来存储和查找字符，右指针一直走到有重复字符时，依次移除左边字符直
    // 到无重复
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();

        if len <= 1 {
            return len as i32;
        }

        let mut recorder = HashSet::new();
        let bytes = s.as_bytes();

        let mut ans = 0;
        let mut right = 0;

        for i in 0..bytes.len() {
            if i != 0 {
                recorder.remove(&bytes[i - 1]);
            }

            while right < len && !recorder.contains(&bytes[right]) {
                recorder.insert(bytes[right]);
                right += 1
            }

            ans = ans.max(right - i);

            // 指针已经到最右，长度不会再增加了
            if right == len {
                break;
            }
        }

        ans as i32
    }
}

struct Solution {}

use std::collections::HashMap;

impl Solution2 {
    // 基本思路就是维护一个不重复的字符串起点start，每次计算最长字串的长度都是以这个字符串作为
    // 字符串起始点；
    // 用一个HashMap来记录和查找字符上次出现的index，出现重复后更新start
    // 注意start只能增大不能减少，因为可能出现类似"abccba"这种字符串
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut start = 0;

        let bytes = s.as_bytes();
        let mut recorder = HashMap::new();

        for i in 0..bytes.len() {
            if let Some(pre_index) = recorder.get_mut(&bytes[i]) {
                start = start.max(*pre_index + 1);
                ans = ans.max(i - start + 1);

                *pre_index = i;
            } else {
                ans = ans.max(i - start + 1);
                recorder.insert(bytes[i], i);
            }
        }

        ans as i32
    }
}

struct Solution2 {}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_solution(solution: fn(s: String) -> i32) {
        let inputs = vec!["abccba", "abacabcbb", "bbbbb", "pwwkew", "", " "];
        let outputs = vec![3, 3, 1, 3, 0, 1];

        for i in 0..inputs.len() {
            assert_eq!(solution(inputs[i].to_owned()), outputs[i]);
        }
    }

    #[test]
    pub fn test() {
        test_solution(Solution::length_of_longest_substring);
        test_solution(Solution2::length_of_longest_substring);
    }
}
