impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }

        let bytes = s.as_bytes();
        let mut start = 0;
        let mut max_len = 1;
        let mut dp = vec![vec![false; len]; len];

        for i in 0..len {
            dp[i][i] = true;
        }

        for l in 2..=len {
            for i in 0..len {
                let j = l + i - 1;

                if j >= len {
                    break;
                }

                if bytes[i] == bytes[j] {
                    dp[i][j] = if j - i < 3 { true } else { dp[i + 1][j - 1] }
                } else {
                    dp[i][j] = false;
                }

                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    start = i;
                }
            }
        }

        s.as_str()[start..start + max_len].to_string()
    }
}

pub struct Solution {}
