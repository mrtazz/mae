use std::path::Path;
// list of characters taken from the wikipedia reserved characters list as a first attempt at a
// quick sanitization method. The "." is removed from the list as that is generally less of a
// problem and that way the method can take a full filename with suffix without having to
// specifically handle the suffix
//
// https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words
//
// We are also cutting the filname of at 255 (which is just short of the classical MAX_PATH of 260)
const MAX_LENGTH: usize = 255;
const PROHIBITED_CHARACTERS: &'static [&'static str] =
    &["/", "\\", "?", "%", "*", ":", "|", "\"", "<", ">", " "];

pub fn sanitize_filename(input: &String) -> String {
    let mut ret_string = String::from("");

    let mut suffix = String::from("");
    let mut usable_max_length = MAX_LENGTH;

    // make sure we retain file extensions

    match Path::new(input).extension() {
        Some(extension) => match extension.to_str() {
            Some(extension) => {
                let suffix_with_dot = &format!(".{}", extension);
                usable_max_length = MAX_LENGTH - suffix_with_dot.len();
                suffix.push_str(suffix_with_dot);
            }
            None => {}
        },
        None => {}
    }

    for (i, c) in input.chars().enumerate() {
        if i >= usable_max_length {
            // make sure we conserve the suffix
            ret_string.push_str(&suffix);
            break;
        }
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
        assert_eq!(
            "a".repeat(MAX_LENGTH - 4) + ".pdf",
            sanitize_filename(&("a".repeat(300) + ".pdf"))
        );
    }
}
