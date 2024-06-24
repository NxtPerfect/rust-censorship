// Implement for multiple rules
pub fn censor_into_nice_word<'a>(text: String, rules: Vec<(&'a str, &'a str)>) -> String {
    let mut res: &str = "";
    let split_text: Vec<&str> = text.split(' ').collect();
    let mut temp: String;
    for mut word in split_text {
        println!("{}", word);
        for rule in &rules {
            if word == rule.0 {
                println!("In rules!");
                word = rule.1;
            }
        }
        temp = format!("{res} {word}");
        res = temp.trim();
    }
    String::from(res)
}

pub fn censor_into_stars(text: String, rules: Vec<&str>) -> String {
    let mut res: &str = "";
    let split_text: Vec<&str> = text.split(' ').collect();
    let mut temp: String;
    for mut word in split_text {
        println!("{}", word);
        for rule in &rules {
            if word.eq(*rule) {
                println!("In rules!");
                word = "****";
            }
        }
        temp = format!("{res} {word}");
        res = temp.trim();
    }
    String::from(res)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_censor() {
        let result = censor_into_nice_word(String::from("ditch"), vec![("ditch", "puppy")]);
        assert_eq!(result, "puppy");
    }

    #[test]
    fn test_simple_censor_stars() {
        let result = censor_into_stars(String::from("ditch"), vec!["ditch"]);
        assert_eq!(result, "****");
    }

    #[test]
    fn test_multiple_censor() {
        let result = censor_into_nice_word(
            String::from("Here is a very duck long ditch string whole"),
            vec![("duck", "quack"), ("ditch", "litch"), ("whole", "ole")],
        );
        assert_eq!(result, "Here is a very quack long litch string ole");
    }

    #[test]
    fn test_multiple_rules() {
        let result = censor_into_nice_word(
            String::from("Here is a very duck long ditch string whole"),
            vec![("duck", "quack"), ("ditch", "litch"), ("whole", "ole"), ("this", "is"), ("a test", "test"), ("for getting", "getting"), ("more", "more"), ("rules", "rules"), ("Here", "here"), ("long", "gong")],
        );
        assert_eq!(result, "here is a very quack gong litch string ole");
    }
}
