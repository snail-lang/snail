use std::rc::Rc;

mod snail;
use snail::*;

fn main() {
    let test = r#"
when := {
  |true body| body!
  |false _|
}

while := {
  |condition body| when condition!, {
    body 1
    while condition, body
  }
}
    "#;

    let lexer = lexer(&mut test.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);
    
    let symtab  = Rc::new(SymTab::new_global());
    let typetab = Rc::new(TypeTab::new_global());
    
    match parser.parse() {
        Ok(n)  => {
            for s in n.iter() {
                match s.visit(&symtab, &typetab) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("{}", e);
                        return
                    },
                }
            }

            println!("{:#?}", n);
            
            for s in n.iter() {
                println!("{}", s)
            }
        },

        Err(e) => println!("{}", e),
    }
}
