// 预处理字符串为
// 字符中间插入 '#'，头部插入 '^'，尾部插入 '$'
// 变成 "^#a#b#a#c#b#$" 的形式
fn pre_process(s: &String) -> String {
    let mut res = "^".to_owned();
    for i in s.chars() {
        res.push('#');
        res.push(i);
    }
    res.push('#');
    res.push('$');

    res
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let t = pre_process(&s);
        let len = t.len();
        let bytes = t.as_bytes();

        // 每个字符最大扩展长度
        let mut expand_counts = vec![0; len];
        // 回文串最大右边界
        let mut right = 0;
        // 拥有最大有边界回文串的中心字符的index
        let mut center = 0;

        let mut max_len = 0;
        let mut max_len_center = 0;

        for i in 1..len - 1 {
            // 首先初始化当前位置的expand_count
            // 如果没有超出最大右边界，初始值为对称点字符的expand_count
            if right > i {
                let i_mirror = 2 * center - i;
                // 注意这里要避免超出右边界
                expand_counts[i] = (right - i).min(expand_counts[i_mirror])
            };

            // 然后继续做中心扩展
            while bytes[i + expand_counts[i] + 1] == bytes[i - expand_counts[i] - 1] {
                expand_counts[i] += 1;
            }

            // 更新最大右边界
            if i + expand_counts[i] > right {
                center = i;
                right = i + expand_counts[i];
            }

            // 更新最大长度
            if expand_counts[i] > max_len {
                max_len = expand_counts[i];
                max_len_center = i;
            }
        }

        // 原始字符串中的下标
        let start = (max_len_center - max_len) / 2;

        s.as_str()[start..start + max_len].to_owned()
    }
}

pub struct Solution {}
