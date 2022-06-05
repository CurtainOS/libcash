#[cfg(test)]
mod tests {
    #[test]
    fn remove_quotes_in_argument() {
        use libcash::split;
        let input = "ls \"hello\"";
        let expected_result = vec!["ls", "hello"];
        assert_eq!(split(input.to_owned()).unwrap(), expected_result)


    }
    #[test]
    fn keep_whitespaces_in_quotes() {
        use libcash::split;
        let input = "booting.. \"hello world\"";
        let expected_result = vec!["booting..", "hello world"];
        assert_eq!(split(input.to_owned()).unwrap(), expected_result)


    }
    #[test]
    #[should_panic]
    fn panic_on_outbalanced_quotes_1() {
        use libcash::split;
        let input = "booting.. \"hello\"world\"";
        let _ = split(input.to_owned()).unwrap();




    }
    #[test]
    #[should_panic]
    fn panic_on_outbalanced_quotes_2() {
        use libcash::split;
        let input = "booting.. \"hello\" there\"world\"";
        let _ = split(input.to_owned()).unwrap();



    }


}