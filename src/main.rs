use std::env;

fn make_text(args: Vec<String>) -> String {
    let text = args.join(" ");

    let out: String = text
        .chars()
        .map(|x| match x {
            '?' => ' ',
            '.' => ' ',
            '/' => ' ',
            ',' => ' ',
            _ => x,
        })
        .collect();

    out.split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
        .to_lowercase()
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_text() {
        let want = String::from("gglong-ggsmall");

        let got = make_text(vec_of_strings!["gglong", "ggsmall"]);
        assert_eq!(want, got);

        let got = make_text(vec_of_strings!["gglong/.ggsmall"]);
        assert_eq!(want, got);

        let got = make_text(vec_of_strings!["gglong?./ggsmall"]);
        assert_eq!(want, got);

        let got = make_text(vec_of_strings!["GglonG?./GGSMALL"]);
        assert_eq!(want, got);
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args = args[1..].to_vec();
    let out = make_text(args);

    println!("{}", out);
}
