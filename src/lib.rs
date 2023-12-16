use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mermaid.pest"]
pub struct MParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p1 = MParser::parse(Rule::node, "x{1}");
        let p2 = MParser::parse(Rule::flowchart, "x{1} --> y");
    }
}
