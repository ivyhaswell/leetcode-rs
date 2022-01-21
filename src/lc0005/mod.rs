mod solution1;
mod solution2;
mod solution3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_solution(solution1::Solution::longest_palindrome);
        test_solution(solution2::Solution::longest_palindrome);
        test_solution(solution3::Solution::longest_palindrome);
    }

    fn test_solution(solution: fn(s: String) -> String) {
        assert_eq!(solution("babad".to_owned()), "bab".to_owned());
        assert_eq!(solution("cbbd".to_owned()), "bb".to_owned());
        assert_eq!(solution("a".to_owned()), "a".to_owned());
        assert_eq!(solution("ac".to_owned()), "a".to_owned());
    }
}
