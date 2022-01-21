fn expand_around_center(bytes: &[u8], mut left: i32, mut right: i32) -> (i32, i32) {
    // 因为usize不能为负数，但是left可能越界，所以这里做一个转换
    while left >= 0 && right < bytes.len() as i32 && bytes[left as usize] == bytes[right as usize] {
        left -= 1;
        right += 1;
    }

    (left + 1, right - 1)
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }

        let bytes = s.as_bytes();
        let mut start = 0;
        let mut end = 0;

        for i in 0..len - 1 {
            let i = i as i32;
            let (left1, right1) = expand_around_center(bytes, i, i);
            if right1 - left1 > end - start {
                start = left1;
                end = right1;
            }

            let (left2, right2) = expand_around_center(bytes, i, i + 1);
            if right2 - left2 > end - start {
                start = left2;
                end = right2;
            }
        }

        s.as_str()[start as usize..=end as usize].to_string()
    }
}

pub struct Solution {}
