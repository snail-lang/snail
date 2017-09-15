mod snail;

use snail::lexer;

fn main() {
    let test = r#"
a: num = 10
    "#;
    
    let lexer = lexer(&mut test.chars());
    
    for token in lexer {
        println!("{:#?}", token)
    }
}
