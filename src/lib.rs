
pub fn split(input: String) -> Result<Vec<String>, std::io::Error> {
    let mut return_vec: Vec<String> = Vec::new();
    let mut temporary_vec: Vec<String> = Vec::new();
    let mut is_this_in_quotes = false;

    for i in input.split_whitespace() {
        if is_this_in_quotes {
            if i.ends_with('"') {
                is_this_in_quotes = false;
                let last_off = &i[0..i.len() - 1];
                temporary_vec.push( " ".to_owned() + last_off);
                let to_append = vec_to_string(&temporary_vec);
                println!("{}", to_append);
                return_vec.push(to_append);
                temporary_vec.clear();
                
            } else {
                let return_str = " ".to_owned() + i;
                temporary_vec.push(return_str);
            }

        }
        else if i.starts_with('"') {
                is_this_in_quotes = true;
                let first_off: &str = &i[1..i.len()];
                temporary_vec.push(first_off.to_owned());
                continue

            } else {
                return_vec.push(i.to_owned());
            }
        

    }



fn vec_to_string(input: &Vec<String>) -> String {
    let mut return_string = String::new();
    for i in input {
        return_string.push_str(i);
  }
    return_string
}
   Ok(return_vec)
 

}



