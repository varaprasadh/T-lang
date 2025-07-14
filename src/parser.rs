use crate::ast::*;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        Ok(Program { statements })
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        // Check for variable declaration
        if self.check_identifier("sankhya") || self.check_identifier("aksharalu") {
            return self.parse_var_declaration();
        }
        
        // Check for keywords
        match &self.peek().token_type {
            TokenType::Ayite => self.parse_if_statement(),
            TokenType::Varaku => self.parse_while_statement(),
            TokenType::Prathi => self.parse_for_statement(),
            TokenType::Pani => self.parse_function_declaration(),
            TokenType::TirigiPampu => self.parse_return_statement(),
            TokenType::Aagu => {
                self.advance();
                self.consume_semicolon()?;
                Ok(Stmt::Aagu)
            }
            TokenType::Kalugu => {
                self.advance();
                self.consume_semicolon()?;
                Ok(Stmt::Kalugu)
            }
            TokenType::Cheppu => self.parse_print_statement(),
            TokenType::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_var_declaration(&mut self) -> Result<Stmt, String> {
        let var_type = match &self.advance().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Expected type name".to_string()),
        };
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Expected variable name".to_string()),
        };
        
        let initializer = if self.match_token(&TokenType::Equal) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume_semicolon()?;
        
        Ok(Stmt::VarDecl { var_type, name, initializer })
    }
    
    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'ayite'
        
        self.consume(&TokenType::LeftParen, "Expected '(' after 'ayite'")?;
        let condition = self.parse_expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after condition")?;
        
        let then_branch = Box::new(self.parse_statement()?);
        
        let else_branch = if self.match_token(&TokenType::Lekapothe) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        
        Ok(Stmt::Ayite { condition, then_branch, else_branch })
    }
    
    fn parse_while_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'varaku'
        
        self.consume(&TokenType::LeftParen, "Expected '(' after 'varaku'")?;
        let condition = self.parse_expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after condition")?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Stmt::Varaku { condition, body })
    }
    
    fn parse_for_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'prathi'
        
        self.consume(&TokenType::LeftParen, "Expected '(' after 'prathi'")?;
        
        // Parse variable name
        let variable = match &self.advance().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Expected variable name in for loop".to_string()),
        };
        
        // Expect 'nunchi' (from) keyword
        if !self.match_token(&TokenType::Nunchi) {
            return Err("Expected 'nunchi' (from) in for loop".to_string());
        }
        
        // Parse iterable expression (range or collection)
        let iterable = self.parse_expression()?;
        
        self.consume(&TokenType::RightParen, "Expected ')' after for loop range")?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Stmt::Prathi { variable, iterable, body })
    }
    
    fn parse_function_declaration(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'pani'
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        
        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_type = match &self.advance().token_type {
                    TokenType::Identifier(s) => s.clone(),
                    _ => return Err("Expected parameter type".to_string()),
                };
                
                let param_name = match &self.advance().token_type {
                    TokenType::Identifier(s) => s.clone(),
                    _ => return Err("Expected parameter name".to_string()),
                };
                
                params.push((param_type, param_name));
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Stmt::Pani { name, params, return_type: None, body })
    }
    
    fn parse_return_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'tirigi_pampu'
        
        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume_semicolon()?;
        
        Ok(Stmt::TirigiPampu(value))
    }
    
    fn parse_print_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'cheppu'
        
        let expr = self.parse_expression()?;
        self.consume_semicolon()?;
        
        Ok(Stmt::Cheppu(expr))
    }
    
    fn parse_block_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume '{'
        
        let mut statements = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after block")?;
        
        Ok(Stmt::Block(statements))
    }
    
    fn parse_expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Expression(expr))
    }
    
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;
        
        if self.match_token(&TokenType::Equal) {
            if let Expr::Variable(name) = expr {
                let value = Box::new(self.parse_assignment()?);
                return Ok(Expr::Assignment { name, value });
            } else {
                return Err("Invalid assignment target".to_string());
            }
        }
        
        Ok(expr)
    }
    
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and()?;
        
        while self.match_token(&TokenType::LedaOp) {
            let right = Box::new(self.parse_and()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Leda,
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_equality()?;
        
        while self.match_token(&TokenType::Mariyu) {
            let right = Box::new(self.parse_equality()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Mariyu,
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;
        
        while let Some(op) = self.match_tokens(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator = match op {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = Box::new(self.parse_comparison()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;
        
        while let Some(op) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = match op {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            let right = Box::new(self.parse_term()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        
        while let Some(op) = self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match op {
                TokenType::Plus => BinaryOp::Plus,
                TokenType::Minus => BinaryOp::Minus,
                _ => unreachable!(),
            };
            let right = Box::new(self.parse_factor()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        
        while let Some(op) = self.match_tokens(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let operator = match op {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = Box::new(self.parse_unary()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(op) = self.match_tokens(&[TokenType::Minus, TokenType::Kadu]) {
            let operator = match op {
                TokenType::Minus => UnaryOp::Minus,
                TokenType::Kadu => UnaryOp::Kadu,
                _ => unreachable!(),
            };
            let operand = Box::new(self.parse_unary()?);
            return Ok(Expr::Unary { operator, operand });
        }
        
        self.parse_call()
    }
    
    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.match_token(&TokenType::LeftParen) {
                // Function call
                if let Expr::Variable(name) = expr {
                    let mut args = Vec::new();
                    
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;
                    expr = Expr::Call { name, args };
                } else {
                    return Err("Invalid function call".to_string());
                }
            } else if self.match_token(&TokenType::LeftBracket) {
                // Array indexing
                let index = Box::new(self.parse_expression()?);
                self.consume(&TokenType::RightBracket, "Expected ']' after array index")?;
                expr = Expr::Index { array: Box::new(expr), index };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.peek().token_type {
            TokenType::Nijam => {
                self.advance();
                Ok(Expr::Nijam)
            }
            TokenType::Abaddham => {
                self.advance();
                Ok(Expr::Abaddham)
            }
            TokenType::Sunya => {
                self.advance();
                Ok(Expr::Sunya)
            }
            TokenType::Sankhya(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Sankhya(num))
            }
            TokenType::Aksharalu(s) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::Aksharalu(string))
            }
            TokenType::Identifier(s) => {
                let name = s.clone();
                self.advance();
                Ok(Expr::Variable(name))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance(); // consume '['
                let mut elements = Vec::new();
                
                if !self.check(&TokenType::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenType::RightBracket, "Expected ']' after array elements")?;
                Ok(Expr::Array(elements))
            }
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }
    
    // Helper methods
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn match_tokens(&mut self, types: &[TokenType]) -> Option<TokenType> {
        for t in types {
            if self.check(t) {
                let token_type = self.peek().token_type.clone();
                self.advance();
                return Some(token_type);
            }
        }
        None
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn check_identifier(&self, name: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        match &self.peek().token_type {
            TokenType::Identifier(s) => s == name,
            _ => false,
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(message.to_string())
        }
    }
    
    fn consume_semicolon(&mut self) -> Result<(), String> {
        self.consume(&TokenType::Semicolon, "Expected ';'")?;
        Ok(())
    }
}