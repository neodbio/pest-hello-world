use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MathParser;

pub fn parse_and_print(input: &str) {
    match MathParser::parse(Rule::program, input) {
        Ok(pairs) => {
            for pair in pairs {
                println!("{:#?}", pair);
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse_and_print("2 + 3 * (4 - 1)");
    }
}