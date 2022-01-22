impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ans = 0;
        let mut x = x;

        while x != 0 {
            // 判断一下是否会溢出
            if ans < i32::MIN / 10 || ans > i32::MAX / 10 {
                return 0;
            }

            ans *= 10;
            ans += x % 10;
            x /= 10;
        }

        ans
    }
}

pub struct Solution {}
