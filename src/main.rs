extern crate structopt;
use structopt::StructOpt;

fn make_text(text: String, from: String, to: String, lowercase: bool, special: bool) -> String {
    let mut out = text;
    if special {
        out = out.replace(
            &['(', ')', ',', '\"', '.', ';', ':', '\'', '!', '/', '?'][..],
            "",
        );
    }

    let mut ret = out.split(&from).collect::<Vec<&str>>().join(&to);

    if lowercase {
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
            false,
        );
        assert_eq!(want, got);

        let want = String::from("gglong-ggsmall");
        let got = make_text(
            String::from("GGLONG_GGSMALL"),
            String::from("_"),
            String::from("-"),
            true,
            false,
        );
        assert_eq!(want, got);

        let want = String::from("gglong\\ ggsmall");
        let got = make_text(
            String::from("gglong ggsmall"),
            String::from(" "),
            String::from("\\ "),
            true,
            false,
        );
        assert_eq!(want, got);

        let want = String::from("gglong-ggsmall");
        let got = make_text(
            String::from("gglong. ggsmall"),
            String::from(" "),
            String::from("-"),
            true,
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
    #[structopt(short, long)]
    special: bool,
}

fn main() {
    let opt = Opt::from_args();
    let out = make_text(
        opt.input_words,
        opt.words_from,
        opt.words_to,
        opt.lowercase,
        opt.special,
    );
    println!("{}", out);
}
