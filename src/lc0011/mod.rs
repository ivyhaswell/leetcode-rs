mod solution1;
// mod solution2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_solution(solution1::Solution::max_area);
        // test_solution(solution2::Solution::my_atoi);
    }

    fn test_solution(solution: fn(height: Vec<i32>) -> i32) {
        assert_eq!(solution(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(solution(vec![1, 1]), 1);
        assert_eq!(solution(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(solution(vec![1, 2, 1]), 2);
    }
}
