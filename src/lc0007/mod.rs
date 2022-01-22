mod solution1;
mod solution2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_solution(solution1::Solution::reverse);
        test_solution(solution2::Solution::reverse);
    }

    fn test_solution(solution: fn(x: i32) -> i32) {
        assert_eq!(solution(123), 321);
        assert_eq!(solution(-123), -321);
        assert_eq!(solution(120), 21);
        assert_eq!(solution(0), 0);
        assert_eq!(solution(-2147483648), 0);
    }
}
