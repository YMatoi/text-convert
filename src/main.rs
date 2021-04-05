#[macro_use]
extern crate clap;

use clap::Arg;

use kanaria::string::{UCSStr, ConvertType};
use kanaria::utils::ConvertTarget;

arg_enum!{
    #[derive(Debug)]
    pub enum Type {
        UpperCase,
        LowerCase,
        Hiragana,
        Katakana,
        Narrow,
        Wide,
        None,
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg_from_usage("<type> 'the type'")
        .arg(
            Arg::with_name("text")
                .multiple(false)
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let text = matches.value_of("text").unwrap().chars().collect::<Vec<_>>();
    let t = value_t!(matches, "type", Type).unwrap_or_else(|e| e.exit());

    let converted = UCSStr::convert(&text, match t {
        Type::UpperCase => ConvertType::UpperCase,
        Type::LowerCase => ConvertType::LowerCase,
        Type::Hiragana => ConvertType::Hiragana,
        Type::Katakana => ConvertType::Katakana,
        Type::Narrow => ConvertType::Narrow,
        Type::Wide => ConvertType::Wide,
        Type::None => ConvertType::None,
    }, ConvertTarget::ALL);

    let result: String = converted.into_iter().collect();

    println!("{}", result);
}
