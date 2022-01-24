// 动态规划
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();

        let schars: Vec<char> = s.chars().collect();
        let pchars: Vec<char> = p.chars().collect();

        let matches = |i: usize, j: usize| -> bool {
            if i == 0 {
                return false;
            }
            if pchars[j - 1] == '.' {
                return true;
            }
            schars[i - 1] == pchars[j - 1]
        };

        // f[i][j]表示s的前i个字符和p的前j个字符是否能匹配
        let mut f = vec![vec![false; n + 1]; m + 1];
        f[0][0] = true;

        for i in 0..=m {
            for j in 1..=n {
                if matches(i, j) {
                    f[i][j] |= f[i - 1][j - 1];
                    continue;
                }
                if pchars[j - 1] == '*' {
                    f[i][j] |= f[i][j - 2];
                    if matches(i, j - 1) {
                        f[i][j] |= f[i - 1][j]
                    }
                }
            }
        }

        f[m][n]
    }
}

pub struct Solution {}
