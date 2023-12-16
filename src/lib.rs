use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mermaid.pest"]
pub struct MParser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
