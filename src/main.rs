extern crate hlua;
use hlua::Lua;

extern crate rustyline;
use rustyline::error::ReadlineError;

use std::rc::Rc;

use std::io;
use std::io::BufRead;

mod snail;
use snail::*;

use std::io::prelude::*;
use std::error::Error;

use std::fs;
use std::fs::File;
use std::fs::metadata;

use std::env;
use std::path::Path;
use std::str::Chars;

#[allow(unused_must_use)]
fn add_global(sym: &SymTab, env: &TypeTab, name: &str, t: Type) {
    let i = sym.add_name(name);

    if i >= env.size() {
        env.grow();
    }

    env.set_type(i, 0, t);
}

fn add_lua_standard(sym: &SymTab, env: &TypeTab) {
    add_global(sym, env, "print",    Type::Block(Rc::new(Type::Any)));
    add_global(sym, env, "read",     Type::Block(Rc::new(Type::Str)));
    add_global(sym, env, "trim",     Type::Block(Rc::new(Type::Str)));
    add_global(sym, env, "tostring", Type::Block(Rc::new(Type::Str)));
}

fn write_path(path: &str) {
    let meta = metadata(path).unwrap();
    
    if meta.is_file() {
        match file(path) {
            Some(n) => write(path, n),
            None    => (),
        }
    } else {
        let paths = fs::read_dir(path).unwrap();

        for path in paths {
            let path = format!("{}", path.unwrap().path().display());
            let split: Vec<&str> = path.split(".").collect();

            match split.get(1) {
                Some(n) if *n == "snail" => (),
                _ => continue,
            }

            write_path(&format!("{}", path))
        }
    }
}

fn execute_path(path_str: &str) {
    let meta = metadata(path_str).unwrap();

    if meta.is_file() {
        match file(path_str) {
            Some(n) => execute(n),
            None    => (),
        }
    } else {
        println!("{}: can't execute folder", path_str)
    }
}

fn file(path: &str) -> Option<Rc<String>> {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why.description()),
        Ok(file) => file,
    };    

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("failed to read {}: {}", display,  why.description()),
        Ok(_)    => transpile(&mut s.chars()),
    }
}

fn transpile(s: &mut Chars) -> Option<Rc<String>> {
    let lexer = lexer(s);

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);
    
    let symtab  = Rc::new(SymTab::new_global());
    let typetab = Rc::new(TypeTab::new_global());
    
    add_lua_standard(&symtab, &typetab);

    match parser.parse() {
        Err(why)  => println!("error: {}", why),
        Ok(stuff) => {                    
            for s in stuff.iter() {
                match s.visit(&symtab, &typetab) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("{}", e);
                        break
                    },
                }
            }

            let mut output = String::new();
            
            for s in stuff.iter() {
                output.push_str(&format!("{}", s))
            }
            
            return Some(Rc::new(output))
        },
    }
    None
}

fn write(path: &str, data: Rc<String>) {
    let path = Path::new(path);
    println!("building: {}", path.display());

    let split_name = path.file_name().unwrap().to_str().unwrap().split(".");
    let split: Vec<&str> = split_name.collect();
    
    let parent_path = match path.parent() {
        Some(p) => match p.file_name() {
            Some(path) => path.to_str().unwrap(),
            None       => ".",
        },
        None => ".",
    };

    let output_name = format!("{}/{}.lua", parent_path, split.get(0).unwrap());

    let mut output_file = File::create(output_name).unwrap();
    match output_file.write_all(data.as_bytes()) {
        Ok(_)    => (),
        Err(why) => println!("{}", why.description())
    }
}

fn execute(data: Rc<String>) {
    let mut lua = Lua::new();
    
    fn print(a: String) {
        println!("{}", a)
    }
    
    fn trim(a: String) -> String {
        a.trim().to_owned()
    }
    
    fn read() -> String {
        let mut line = String::new();
        let stdin    = io::stdin();
        
        stdin.lock().read_line(&mut line).unwrap();
        line
    }

    lua.set("print", hlua::function1(print));
    lua.set("trim",  hlua::function1(trim));
    lua.set("read",  hlua::function0(read));
    
    match lua.execute::<()>(&data) {
        Ok(_)    => (),
        Err(why) => println!("{}", format!("{}", why).to_lowercase()),
    }
}

fn repl() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(">");

        match readline {
            Ok(line) => match transpile(&mut line.chars()) {
                Some(n) => execute(n),
                None    => (),
            },
            Err(ReadlineError::Interrupted) => {
                println!("interrupted");
                break
            }

            Err(ReadlineError::Eof) => {
                println!("eof");
                break
            }

            Err(err) => {
                println!("error: {:?}", err);
                break
            }
        }
    }
}

fn main() {
    match env::args().nth(1) {
        Some(a) => match a.as_str() {
            "c" => {
                write_path(
                    &match env::args().nth(2) {
                        Some(n) => n,
                        None    => {
                            println!("missing supplied path");
                            return
                        }
                    }
                )
            },
            _ => execute_path(&a),
        },
        None => println!("repl is wip"),
    }
}
