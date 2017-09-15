mod snail;

use snail::lexer;
use snail::{Parser, Traveler};

fn main() {
    let test = r#"
a: num = 10
    "#;
    
    let lexer = lexer(&mut test.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);
    
    for s in parser.parse() {
        println!("{:#?}", s)
    }
}
