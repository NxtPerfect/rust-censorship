pub fn censor_into_nice_word<'a>(text: String, rules: Vec<(&'a str, &'a str)>) -> String {
    for r in rules {
        if text.eq(r.0) {
            return String::from(r.1);
        } else {
            return text;
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_simple_censor() {
        let result = censor_into_nice_word(String::from("bitch"), vec![("bitch", "puppy")]);
        assert_eq!(result, "puppy");
    }
}
