impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let m = nums1.len();
        let n = nums2.len();

        if m | n == 0 {
            return 1.0;
        }

        let mut left = 0;
        let mut right = m;
        let mut median1 = 0;
        let mut median2 = 0;

        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;

            let nums_im1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let nums_i = if i == m { i32::MAX } else { nums1[i] };
            let nums_jm1 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let nums_j = if j == n { i32::MAX } else { nums2[j] };

            if nums_im1 <= nums_j {
                median1 = nums_im1.max(nums_jm1);
                median2 = nums_i.min(nums_j);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if (m + n) % 2 == 0 {
            (median1 + median2) as f64 / 2.0
        } else {
            median1 as f64
        }
    }
}

struct Solution {}

pub fn find_kth_element(nums1: &Vec<i32>, nums2: &Vec<i32>, k: usize) -> f64 {
    let len1 = nums1.len();
    let len2 = nums2.len();

    let mut index1 = 0;
    let mut index2 = 0;
    let mut k = k;

    loop {
        if index1 == len1 {
            return nums2[index2 + k - 1] as f64;
        }
        if index2 == len2 {
            return nums1[index1 + k - 1] as f64;
        }
        if k == 1 {
            return nums1[index1].min(nums2[index2]) as f64;
        }

        let half = k / 2;
        let new_index1 = (index1 + half).min(len1) - 1;
        let new_index2 = (index2 + half).min(len2) - 1;

        if nums1[new_index1] <= nums2[new_index2] {
            k -= new_index1 - index1 + 1;
            index1 = new_index1 + 1;
        } else {
            k -= new_index2 - index2 + 1;
            index2 = new_index2 + 1;
        }
    }
}

impl Solution2 {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let total = m + n;

        if m | n == 0 {
            return 1.0;
        }

        if total % 2 == 0 {
            (find_kth_element(&nums1, &nums2, total / 2)
                + find_kth_element(&nums1, &nums2, total / 2 + 1))
                / 2.0
        } else {
            find_kth_element(&nums1, &nums2, total / 2 + 1)
        }
    }
}

struct Solution2 {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(solution: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> f64) {
        assert_eq!(solution(vec![1, 3], vec![2]), 2.0);
        assert_eq!(solution(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(solution(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(solution(vec![], vec![]), 1.0);
        assert_eq!(solution(vec![2], vec![]), 2.0);
    }
    #[test]
    fn test() {
        test_solution(Solution::find_median_sorted_arrays);
        test_solution(Solution2::find_median_sorted_arrays);
    }
}
