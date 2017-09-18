use std::rc::Rc;

use super::*;
use super::lexer::*;

pub struct Parser {
    traveler: Traveler,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(traveler: Traveler) -> Parser {
        Parser {
            traveler,
        }
    }

    pub fn parse(&mut self) -> ParserResult<Vec<Statement>> {
        let mut stack = Vec::new();
        while self.traveler.remaining() > 2 {
            if let Some(s) = self.statement()? {
                stack.push(s);
                self.traveler.next();
            } else {
                stack.push(Statement::Expression(Rc::new(self.expression()?)))
            }
        }

        Ok(stack)
    }

    fn types(&mut self) -> ParserResult<Option<Type>> {
        match self.traveler.current().token_type {
            TokenType::Type   => {
                let t = get_type(&self.traveler.current_content()).unwrap();
                self.traveler.next();

                match self.traveler.current_content().as_str() {
                    ".." => {
                        self.traveler.next();
                        
                        Ok(Some(Type::Array(Rc::new(t))))
                    },

                    _ => Ok(Some(t))
                }
            },
            _ => Ok(None),
        }
    }
    
    pub fn skip_whitespace(&mut self) -> ParserResult<()> {
        while self.traveler.current_content() == "\n" || self.traveler.current().token_type == TokenType::EOL {
            self.traveler.next();
            
            if self.traveler.remaining() < 2 {
                break
            }
        }

        Ok(())
    }
    
    pub fn arm(&mut self) -> ParserResult<Expression> {
        self.traveler.expect_content("|")?;
        self.traveler.next();

        let mut params = Vec::new();
        
        while self.traveler.current_content() != "|" {
            self.skip_whitespace()?;
            
            println!("her: {}", self.traveler.current_content());
            
            let a = match self.traveler.current().token_type {
                TokenType::IntLiteral    => {
                    let a = Expression::Number(self.traveler.current_content().parse::<f64>().unwrap());
                    self.traveler.next();
                    a
                }
                TokenType::FloatLiteral  => {
                    let a = Expression::Number(self.traveler.current_content().parse::<f64>().unwrap());
                    self.traveler.next(); 
                    a
                }
                TokenType::BoolLiteral   => {
                    let a = Expression::Bool(self.traveler.current_content() == "true");
                    self.traveler.next();
                    a
                }
                TokenType::StringLiteral => {
                    let a = Expression::Str(Rc::new(self.traveler.current_content().clone()));
                    self.traveler.next();
                    a
                }
                TokenType::Identifier => {
                    let a = Expression::Identifier(Rc::new(self.traveler.current_content().clone()));
                    self.traveler.next();
                    a
                }
                _ => return Err(ParserError::new_pos(self.traveler.current().position, &format!("expected identifier, str, num or bool, found: {}", self.traveler.current_content())))
            };
            
            params.push(Rc::new(a));
            
            if self.traveler.remaining() < 2 {
                break
            }
        }

        self.traveler.next();
        
        let body = self.expression()?;
        
        Ok(Expression::Arm(params, Rc::new(body)))
    }

    pub fn term(&mut self) -> ParserResult<Expression> {
        self.skip_whitespace()?;
        
        if self.traveler.remaining() < 2 {
            return Ok(Expression::EOF)
        }
        
        match self.traveler.current().token_type {
            TokenType::IntLiteral    => {
                let a = Ok(Expression::Number(self.traveler.current_content().parse::<f64>().unwrap()));
                self.traveler.next();
                a
            }
            TokenType::FloatLiteral  => {
                let a = Ok(Expression::Number(self.traveler.current_content().parse::<f64>().unwrap()));
                self.traveler.next(); 
                a
            }
            TokenType::BoolLiteral   => {
                let a = Ok(Expression::Bool(self.traveler.current_content() == "true"));
                self.traveler.next();
                a
            }
            TokenType::StringLiteral => {
                let a = Ok(Expression::Str(Rc::new(self.traveler.current_content().clone())));
                self.traveler.next();
                a
            }
            TokenType::Symbol => match self.traveler.current_content().as_str() {
                "{" => {
                    self.traveler.next();
                    self.block()
                },
                "|" => self.arm(),
                "(" => {
                    self.traveler.next();
                    if self.traveler.current_content() == ")" {
                        return Err(ParserError::new_pos(self.traveler.current().position, &format!("illegal empty clause '()'")))
                    }
                    
                    let expr = self.expression()?;

                    self.skip_whitespace()?;
                    self.traveler.expect_content(")")?;
                    self.traveler.next();

                    match self.traveler.current().token_type {
                        TokenType::IntLiteral |
                        TokenType::FloatLiteral |
                        TokenType::BoolLiteral |
                        TokenType::StringLiteral |
                        TokenType::Identifier |
                        TokenType::Symbol => {
                            if self.traveler.current().token_type == TokenType::Symbol {
                                match self.traveler.current_content().as_str() {
                                    "!"  => {
                                        self.traveler.next();
                                        return Ok(Expression::Call(Rc::new(expr), Rc::new(vec!())));
                                    },
                                    "(" => {
                                        let call = self.call(expr)?;
                                        self.traveler.next();
                                        return Ok(call)
                                    },
                                    "=" => {
                                        self.traveler.next();
                                        let expr_right = self.expression()?;

                                        return Ok(Expression::Assignment(Rc::new(expr), Rc::new(expr_right)))
                                    },
                                    _   => return Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected symbol: {}", self.traveler.current_content()))),
                                }
                            }
                            let call = self.call(expr)?;
                            //self.traveler.next();
                            return Ok(call)
                        },
                        _ => (),
                    }

                    Ok(expr)
                },
                _ => Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected symbol: {}", self.traveler.current_content()))),
            },
            TokenType::Identifier => {
                let id = Expression::Identifier(Rc::new(self.traveler.current_content()));
                self.traveler.next();
                
                match self.traveler.current().token_type {
                    TokenType::IntLiteral |
                    TokenType::FloatLiteral |
                    TokenType::BoolLiteral |
                    TokenType::StringLiteral |
                    TokenType::Identifier => {
                        let call = self.call(id)?;

                        Ok(call)
                    },

                    TokenType::Symbol => match self.traveler.current_content().as_str() {
                        ")" | "," => {
                            self.traveler.next();
                            Ok(id)
                        },
                        "}" | "|" => Ok(id),
                        "("       => Ok(self.call(id)?),
                        "!"       => {
                            self.traveler.next();
                            Ok(Expression::Call(Rc::new(id), Rc::new(vec!())))
                        },
                        "="       => {                            
                            self.traveler.next();
                            let expr = self.expression()?;

                            Ok(Expression::Assignment(Rc::new(id), Rc::new(expr)))
                        },

                        _ => Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected: {}", self.traveler.current_content()))),
                    },
                    _ => Ok(id),
                }
            },
            _ => Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected: {}", self.traveler.current_content()))),
        }
    }
    
    fn block(&mut self) -> ParserResult<Expression> {
        if self.traveler.current_content() == "}" {
            return Err(ParserError::new_pos(self.traveler.current().position, &format!("illegal empty clause '{{}}'")))
        }

        let mut body = Vec::new();
        
        while self.traveler.current_content() != "}" {
            body.push(self.traveler.current().clone());
            self.traveler.next();
            self.skip_whitespace()?;
        }
        
        self.traveler.next();
        
        body.push(Token::new(TokenType::EOL, TokenPosition::new(0, 0), "\n".to_owned()));
        
        let traveler   = Traveler::new(body);
        let mut parser = Parser::new(traveler);
        
        Ok(Expression::Block(parser.parse()?))
    }

    fn expression(&mut self) -> ParserResult<Expression> {
        self.skip_whitespace()?;
        
        let expr = self.term()?;
        
        if expr == Expression::EOF {
            return Ok(expr)
        }

        if self.traveler.remaining() > 0 {
            self.skip_whitespace()?;
            if self.traveler.current().token_type == TokenType::Operator {
                return self.operation(expr)
            }
        }

        Ok(expr)
    }

    pub fn statement(&mut self) -> ParserResult<Option<Statement>> {
        self.skip_whitespace()?;
        match self.traveler.current().token_type {
            TokenType::Symbol => match self.traveler.current_content().as_str() {
                "\n" => {
                    self.traveler.next();
                    self.statement()
                },
                _ => Ok(None),
            },
            TokenType::Identifier => {
                let id = self.traveler.current_content();
                self.traveler.next();
                
                if self.traveler.current_content() == ":" {
                    self.traveler.next();
                    
                    if let Some(t) = self.types()? {
                        match self.traveler.current_content().as_str() {
                            "=" => {
                                self.traveler.next();
                                let expr = self.expression()?;
                                
                                Ok(Some(Statement::Definition(Some(t), Rc::new(id), Some(Rc::new(expr)))))
                            },

                            _ => Ok(Some(Statement::Definition(Some(t), Rc::new(id), None))),
                        }
                    } else if self.traveler.current_content() == "=" {
                        self.traveler.next();
                        Ok(Some(Statement::Definition(None, Rc::new(id), Some(Rc::new(self.expression()?)))))
                    } else {
                        Err(ParserError::new_pos(self.traveler.current().position, &format!("expected '=' or type, found: {}", self.traveler.current_content())))
                    }

                } else {
                    self.traveler.prev();
                    Ok(Some(Statement::Expression(Rc::new(self.expression()?))))
                }
            },
            _ => Ok(Some(Statement::Expression(Rc::new(self.expression()?)))),
        }
    }
    
    fn call(&mut self, caller: Expression) -> ParserResult<Expression> {
        let mut args = Vec::new();

        while self.traveler.current_content() != ")" && self.traveler.current_content() != "}" && self.traveler.current_content() != "\n" {
            args.push(try!(self.expression()));

            if self.traveler.current_content() == "," {
                self.traveler.next();
            }
        }
        
        Ok(Expression::Call(Rc::new(caller), Rc::new(args)))
    }
    
    fn operation(&mut self, expression: Expression) -> ParserResult<Expression> {
        let mut ex_stack = vec![expression];
        let mut op_stack: Vec<(Operand, u8)> = Vec::new();

        op_stack.push(get_operand(&self.traveler.current_content()).unwrap());
        self.traveler.next();

        if self.traveler.current_content() == "\n" {
            self.traveler.next();
        }

        ex_stack.push(self.term()?);
        self.traveler.prev();

        let mut done = false;
        while ex_stack.len() > 1 {
            if !done && self.traveler.next() {
                if self.traveler.current().token_type != TokenType::Operator {
                    self.traveler.prev();
                    done = true;
                    continue
                }

                let (op, precedence) = get_operand(&self.traveler.current_content()).unwrap();
                
                self.traveler.next();

                if precedence >= op_stack.last().unwrap().1 {
                    let left  = ex_stack.pop().unwrap();
                    let right = ex_stack.pop().unwrap();

                    ex_stack.push(Expression::Operation {
                        right: Rc::new(left),
                        op:    op_stack.pop().unwrap().0,
                        left:  Rc::new(right)
                    });

                    ex_stack.push(self.term()?);
                    self.traveler.prev();
                    op_stack.push((op, precedence));

                    continue
                }

                ex_stack.push(self.term()?);
                self.traveler.prev();
                op_stack.push((op, precedence));
            }

            let left  = ex_stack.pop().unwrap();
            let right = ex_stack.pop().unwrap();

            ex_stack.push(Expression::Operation {
                right: Rc::new(left),
                op:    op_stack.pop().unwrap().0,
                left:  Rc::new(right)
            });
        }
        
        self.traveler.next();
        
        Ok(ex_stack.pop().unwrap())
    }
}
