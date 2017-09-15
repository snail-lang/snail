use std::rc::Rc;

use super::*;
use super::lexer::TokenType;

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
    
    pub fn term(&mut self) -> ParserResult<Expression> {
        self.skip_whitespace()?;
        
        match self.traveler.current().token_type {
            TokenType::IntLiteral    => Ok(Expression::Number(self.traveler.current_content().parse::<f64>().unwrap())),
            TokenType::FloatLiteral  => Ok(Expression::Number(self.traveler.current_content().parse::<f64>().unwrap())),
            TokenType::BoolLiteral   => Ok(Expression::Bool(self.traveler.current_content() == "true")),
            TokenType::StringLiteral => Ok(Expression::Str(Rc::new(self.traveler.current_content().clone()))),
            _ => Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected: {}", self.traveler.current_content()))),
        }
    }
    
    fn expression(&mut self) -> ParserResult<Expression> {
        if self.traveler.current_content() == "\n" {
            self.traveler.next();
        }
        
        let expr = self.term()?;

        if expr == Expression::EOF {
            return Ok(expr)
        }

        self.traveler.next();
        
        if self.traveler.remaining() > 0 {
            if self.traveler.current().token_type == TokenType::Operator {
                return self.operation(expr)
            }
            
            self.traveler.prev();
        }

        Ok(expr)
    }
    
    pub fn statement(&mut self) -> ParserResult<Option<Statement>> {
        match self.traveler.current().token_type {
            TokenType::Symbol => match self.traveler.current_content().as_str() {
                "\n" => {
                    self.traveler.next();
                    self.statement()
                },
                c => Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected symbol: {}", c))),
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
                        Ok(Some(Statement::Definition(None, Rc::new(id), Some(Rc::new(self.expression()?)))))
                    } else {
                        Err(ParserError::new_pos(self.traveler.current().position, &format!("expected '=' or type, found: {}", self.traveler.current_content())))
                    }

                } else {
                    Ok(None)
                }
            },
            _ => Err(ParserError::new_pos(self.traveler.current().position, &format!("unexpected: {}", self.traveler.current_content()))),
        }
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
                    op_stack.push((op, precedence));

                    continue
                }

                ex_stack.push(self.term()?);
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
