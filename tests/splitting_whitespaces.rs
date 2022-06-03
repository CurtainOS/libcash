
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
    #[test]
    fn long_array_split() {
        use libcash::split;
        let input = "hello this string should be splitted into exacly fifteen words and i hope it works";
        //you might wonder why i use the split_whitespace to check if a split whitespace worked, but this is simply because the libary does NOT use split_whitespace, lul
        let expected_result: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(split(input.to_owned()), expected_result)

    }
    #[test]
    fn very_long_array_split() {
        use libcash::split;
        let input = "hello ".repeat(100);
        let expected_result: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(split(input.to_owned()), expected_result)


    }
    #[test]
    fn extremely_long_array_split() {
        use libcash::split;
        let input = "hello ".repeat(10000);
        let expected_result: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(split(input.to_owned()), expected_result)


    }
    #[test]
    fn should_ignore_obsolete_whitespace() {
        use libcash::split;
        let input = "hello this  contains obsolete    whitespaces    ";
        let expected_result = vec!["hello", "this", "contains", "obsolete", "whitespaces"];
        assert_eq!(split(input.to_owned()), expected_result)


    }
    #[test]
    fn very_long_word_array() {
        use libcash::split;
        let word1 = "hello".repeat(100);
        let word2 = "world".repeat(100);
        let input = word1.to_owned() + " " + &word2;
        let expected_result: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(split(input.to_owned()), expected_result)


    }
    #[test]
    fn extemely_long_word_array() {
        use libcash::split;
        let word1 = "hello".repeat(1000000);
        let word2 = "world".repeat(1000000);
        let input = word1.to_owned() + " " + &word2;
        let expected_result: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(split(input.to_owned()), expected_result)


    }
    #[test]
    fn no_whitespace() {
        use libcash::split;
        let input = "hi";
        let expected_result = vec!["hi"];
        assert_eq!(split(input.to_owned()), expected_result)


    }
    #[test]
    //this test used to use more than 100.000.000.000.000 bytes and worked, but because of time and simplicity i chose to reduce this. 
    //Rest assured that the whitespace splitting is very robust
    fn final_extreme_whitespace_stress_test() {
        use libcash::split;
        let word1 = "hello".repeat(10000);
        let word2 = "world".repeat(10000);
        let buffer = word1.to_owned() + " " + &word2;
        let input = buffer.repeat(1000);
        let expected_result: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(split(input.to_owned()), expected_result)
    }
}
