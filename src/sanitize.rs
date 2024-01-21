// list of characters taken from the wikipedia reserved characters list as a first attempt at a
// quick sanitization method. The "." is removed from the list as that is generally less of a
// problem and that way the method can take a full filename with suffix without having to
// specifically handle the suffix
//
// https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words
const PROHIBITED_CHARACTERS: &'static [&'static str] =
    &["/", "\\", "?", "%", "*", ":", "|", "\"", "<", ">", " "];

pub fn sanitize_filename(input: &String) -> String {
    let mut ret_string = String::from("");

    for c in input.chars() {
        if PROHIBITED_CHARACTERS.contains(&(c.to_string().as_str())) {
            ret_string = ret_string + "_";
        } else {
            ret_string.push_str(c.to_string().as_str());
        }
    }

    return ret_string;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sanitize() {
        assert_eq!("foo_", sanitize_filename(&String::from("foo:")));
        assert_eq!("f_oo_", sanitize_filename(&String::from("f/oo:")));
        assert_eq!(
            "no_spaces_allowed",
            sanitize_filename(&String::from("no spaces allowed"))
        );
    }
}
