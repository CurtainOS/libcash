
use regex::Regex;
use std::io;
pub fn split(input: String) -> Result<Vec<String>, io::Error> {
    let mut return_vec: Vec<String> = Vec::new();

    for mat in Regex::new(r#""[^"]+"|'[^']+'|[\w'-'/'.]+"#)
        .unwrap()
        .find_iter(input.as_str())
    {
        let  to_append = mat.as_str();

     

     if to_append.starts_with('\'') && to_append.ends_with('\'')|| to_append.starts_with('"') && to_append.ends_with('"'){
            let mut chars = to_append.chars();
            chars.next();
            chars.next_back();
            let to_return = chars.as_str();
            return_vec.push(to_return.to_string());


        }
        else {
            return_vec.push(to_append.to_string());

        }

    }
    Ok(return_vec)

}



