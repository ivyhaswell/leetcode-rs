// 这个方法其实不合题意，使用了64位整形
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // 注意is_positive不会通过0
        let flag = if x.is_negative() { -1 } else { 1 };
        // 注意这里要先转换成i64再用abs，否则x的值为i32::MIN的时候会溢出
        let mut x = (x as i64).abs();

        if x < 10 {
            return x as i32;
        }

        let mut ans = 0;

        while x > 0 {
            ans *= 10;
            ans += x % 10;
            x /= 10;
        }

        ans *= flag;

        if ans > i32::MAX as i64 || ans < i32::MIN as i64 {
            0
        } else {
            ans as i32
        }
    }
}

pub struct Solution {}
