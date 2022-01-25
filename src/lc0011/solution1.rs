// 双指针，实现很简单，关键是证明其正确性
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut l = 0;
        let mut r = len - 1;
        let mut ans = 0;

        while l < r {
            let lh = height[l];
            let rh = height[r];

            if lh < rh {
                ans = ans.max(lh * (r - l) as i32);
                l += 1;
                while l < r && height[l] < height[l - 1] {
                    l += 1;
                }
            } else {
                ans = ans.max(rh * (r - l) as i32);
                r -= 1;
                while l < r && height[r] < height[r + 1] {
                    r -= 1;
                }
            }
        }

        ans
    }
}

pub struct Solution {}
