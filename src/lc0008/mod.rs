mod solution1;
mod solution2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_solution(solution1::Solution::my_atoi);
        test_solution(solution2::Solution::my_atoi);
    }

    fn test_solution(solution: fn(s: String) -> i32) {
        assert_eq!(solution("42".to_owned()), 42);
        assert_eq!(solution("   -42".to_owned()), -42);
        assert_eq!(solution("4193 with words".to_owned()), 4193);
        assert_eq!(solution("words and 987".to_owned()), 0);
        assert_eq!(solution("-91283472332".to_owned()), -2147483648);
        assert_eq!(solution("00000-42a1234".to_owned()), 0);
        assert_eq!(solution("2147483648".to_owned()), 2147483647);
    }
}
