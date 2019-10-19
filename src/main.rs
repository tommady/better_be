extern crate structopt;
use structopt::StructOpt;

fn make_text(text: String, from: String, to: String, to_lowercase: bool) -> String {
    let out: String = text
        .chars()
        .map(|x| match x {
            '?' => ' ',
            '.' => ' ',
            '/' => ' ',
            ',' => ' ',
            '!' => ' ',
            _ => x,
        })
        .collect();

    let str_from = from.to_string();
    let str_to = to.to_string();
    let mut ret = out.split(&str_from).collect::<Vec<&str>>().join(&str_to);

    if to_lowercase {
        ret = ret.to_lowercase();
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_text() {
        let want = String::from("gglong-ggsmall");
        let got = make_text(
            String::from("gglong_ggsmall"),
            String::from("_"),
            String::from("-"),
            false,
        );
        assert_eq!(want, got);

        let want = String::from("gglong-ggsmall");
        let got = make_text(
            String::from("GGLONG_GGSMALL"),
            String::from("_"),
            String::from("-"),
            true,
        );
        assert_eq!(want, got);

        let want = String::from("gglong\\ ggsmall");
        let got = make_text(
            String::from("gglong ggsmall"),
            String::from(" "),
            String::from("\\ "),
            true,
        );
        assert_eq!(want, got);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "better_be", about = "format the words you want")]
struct Opt {
    #[structopt(parse(from_str))]
    input_words: String,
    #[structopt(short = "f", long = "from", default_value = " ")]
    words_from: String,
    #[structopt(short = "t", long = "to", default_value = "\\ ")]
    words_to: String,
    #[structopt(short, long)]
    lowercase: bool,
}

fn main() {
    let opt = Opt::from_args();
    let out = make_text(opt.input_words, opt.words_from, opt.words_to, opt.lowercase);
    println!("{}", out);
}
