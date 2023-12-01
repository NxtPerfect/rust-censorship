// Implement for multiple rules
pub fn censor_into_nice_word<'a>(text: String, rules: Vec<(&'a str, &'a str)>) -> String {
    let mut res: &str = "";
    let text_split: Vec<&str> = text.split(' ').collect();
    let mut together: String = String::from("");
    for r in rules {
        for t in &text_split {
            // if true return r.1, else return t, build together string
            if t == &r.0 {
                together = format!("{} {}", res, r.1);
                res = together.trim();
            } else {
                together = format!("{res} {t}");
                res = together.trim();
            }
        }
    }
    String::from(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_censor() {
        let result = censor_into_nice_word(String::from("bitch"), vec![("bitch", "puppy")]);
        assert_eq!(result, "puppy");
    }

    #[test]
    fn test_multiple_censor() {
        let result = censor_into_nice_word(
            String::from("Here is a very fuck long bitch string whore"),
            vec![("fuck", "duck"), ("bitch", "ditch"), ("whore", "whole")],
        );
        assert_eq!(result, "Here is a very duck long ditch string whole");
    }
}
