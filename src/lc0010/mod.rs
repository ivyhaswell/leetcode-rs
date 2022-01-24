mod solution1;
// mod solution2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_solution(solution1::Solution::is_match);
        // test_solution(solution2::Solution::my_atoi);
    }

    fn test_solution(solution: fn(s: String, p: String) -> bool) {
        assert_eq!(solution("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(solution("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(solution("ab".to_owned(), ".*".to_owned()), true);
        assert_eq!(solution("aab".to_owned(), "c*a*b".to_owned()), true);
        assert_eq!(
            solution("mississippi".to_owned(), "mis*is*p*.".to_owned()),
            false
        );
    }
}
