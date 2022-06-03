
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn simple_whitespace_split() {
        use libcash::split;
        let input = "ls /";
        let expected_result = vec!["ls", "/"];
        assert_eq!(split(input.to_owned()), expected_result)

    }
}
