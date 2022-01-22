impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // 0: 读空格
        // 1: 读符号
        // 2: 读数字
        let mut step = 0;
        let mut flag = 1;
        let mut num = 0;

        for i in s.as_bytes() {
            match i {
                b' ' => {
                    if step != 0 {
                        break;
                    }
                }
                b'+' | b'-' => {
                    if step >= 1 {
                        break;
                    }
                    step = 1;
                    if i == &b'-' {
                        flag = -1;
                    }
                }
                b'.' => {
                    break;
                }
                b'A'..=b'z' => {
                    break;
                }
                b'0'..=b'9' => {
                    step = 2;

                    let digit = (i - b'0') as i32;

                    if num * flag > (i32::MAX - digit) / 10 {
                        return i32::MAX;
                    }
                    if num * flag < (i32::MIN + digit) / 10 {
                        return i32::MIN;
                    }
                    num = num * 10 + digit;
                }
                _ => {}
            }
        }

        num * flag
    }
}

pub struct Solution {}
