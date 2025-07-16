use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut keywords = HashMap::new();
        
        // Telugu keywords
        keywords.insert("sankhya".to_string(), TokenType::Identifier("sankhya".to_string()));
        keywords.insert("aksharalu".to_string(), TokenType::Identifier("aksharalu".to_string()));
        keywords.insert("nijam".to_string(), TokenType::Nijam);
        keywords.insert("abaddham".to_string(), TokenType::Abaddham);
        keywords.insert("sunya".to_string(), TokenType::Sunya);
        keywords.insert("ayite".to_string(), TokenType::Ayite);
        keywords.insert("lekapothe".to_string(), TokenType::Lekapothe);
        keywords.insert("leda".to_string(), TokenType::LedaOp);
        keywords.insert("varaku".to_string(), TokenType::Varaku);
        keywords.insert("prathi".to_string(), TokenType::Prathi);
        keywords.insert("aagu".to_string(), TokenType::Aagu);
        keywords.insert("kalugu".to_string(), TokenType::Kalugu);
        keywords.insert("pani".to_string(), TokenType::Pani);
        keywords.insert("tirigi_pampu".to_string(), TokenType::TirigiPampu);
        keywords.insert("rakam".to_string(), TokenType::Rakam);
        keywords.insert("kotha".to_string(), TokenType::Kotha);
        keywords.insert("sontham".to_string(), TokenType::Sontham);
        keywords.insert("techu".to_string(), TokenType::Techu);
        keywords.insert("nunchi".to_string(), TokenType::Nunchi);
        keywords.insert("penchu".to_string(), TokenType::Penchu);
        keywords.insert("bhagam".to_string(), TokenType::Bhagam);
        keywords.insert("prayatnam".to_string(), TokenType::Prayatnam);
        keywords.insert("tappu".to_string(), TokenType::Tappu);
        keywords.insert("chivara".to_string(), TokenType::Chivara);
        keywords.insert("cheppu".to_string(), TokenType::Cheppu);
        keywords.insert("adugu".to_string(), TokenType::Adugu);
        keywords.insert("pradhaanam".to_string(), TokenType::Pradhaanam);
        keywords.insert("mariyu".to_string(), TokenType::Mariyu);
        keywords.insert("kadu".to_string(), TokenType::Kadu);
        
        Lexer {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }
        
        tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            self.line,
            self.column,
        ));
        
        tokens
    }
    
    fn scan_token(&mut self) -> Option<Token> {
        let start_column = self.column;
        let c = self.advance();
        
        match c {
            '+' => Some(self.make_token(TokenType::Plus, "+", start_column)),
            '-' => Some(self.make_token(TokenType::Minus, "-", start_column)),
            '*' => Some(self.make_token(TokenType::Star, "*", start_column)),
            '/' => {
                if self.peek() == '/' {
                    self.skip_comment();
                    None
                } else {
                    Some(self.make_token(TokenType::Slash, "/", start_column))
                }
            }
            '%' => Some(self.make_token(TokenType::Percent, "%", start_column)),
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(self.make_token(TokenType::EqualEqual, "==", start_column))
                } else {
                    Some(self.make_token(TokenType::Equal, "=", start_column))
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(self.make_token(TokenType::NotEqual, "!=", start_column))
                } else {
                    None
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(self.make_token(TokenType::LessEqual, "<=", start_column))
                } else {
                    Some(self.make_token(TokenType::Less, "<", start_column))
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(self.make_token(TokenType::GreaterEqual, ">=", start_column))
                } else {
                    Some(self.make_token(TokenType::Greater, ">", start_column))
                }
            }
            '(' => Some(self.make_token(TokenType::LeftParen, "(", start_column)),
            ')' => Some(self.make_token(TokenType::RightParen, ")", start_column)),
            '{' => Some(self.make_token(TokenType::LeftBrace, "{", start_column)),
            '}' => Some(self.make_token(TokenType::RightBrace, "}", start_column)),
            '[' => Some(self.make_token(TokenType::LeftBracket, "[", start_column)),
            ']' => Some(self.make_token(TokenType::RightBracket, "]", start_column)),
            ',' => Some(self.make_token(TokenType::Comma, ",", start_column)),
            ';' => Some(self.make_token(TokenType::Semicolon, ";", start_column)),
            ':' => Some(self.make_token(TokenType::Colon, ":", start_column)),
            '"' => self.scan_string(start_column),
            _ if c.is_numeric() => self.scan_number(start_column),
            _ if c.is_alphabetic() || c == '_' => self.scan_identifier(start_column),
            _ => None,
        }
    }
    
    fn scan_string(&mut self, start_column: usize) -> Option<Token> {
        let mut value = String::new();
        
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            value.push(self.advance());
        }
        
        if self.is_at_end() {
            return None;
        }
        
        self.advance(); // Closing "
        
        Some(Token::new(
            TokenType::Aksharalu(value.clone()),
            format!("\"{}\"", value),
            self.line,
            start_column,
        ))
    }
    
    fn scan_number(&mut self, start_column: usize) -> Option<Token> {
        let mut value = String::new();
        value.push(self.previous());
        
        while self.peek().is_numeric() {
            value.push(self.advance());
        }
        
        if self.peek() == '.' && self.peek_next().is_numeric() {
            value.push(self.advance()); // .
            
            while self.peek().is_numeric() {
                value.push(self.advance());
            }
        }
        
        if let Ok(num) = value.parse::<f64>() {
            Some(Token::new(
                TokenType::Sankhya(num),
                value,
                self.line,
                start_column,
            ))
        } else {
            None
        }
    }
    
    fn scan_identifier(&mut self, start_column: usize) -> Option<Token> {
        let mut value = String::new();
        value.push(self.previous());
        
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            value.push(self.advance());
        }
        
        let token_type = self.keywords.get(&value)
            .cloned()
            .unwrap_or_else(|| TokenType::Identifier(value.clone()));
        
        Some(Token::new(token_type, value, self.line, start_column))
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 0;
                }
                _ => break,
            }
        }
    }
    
    fn skip_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }
    
    fn advance(&mut self) -> char {
        let c = self.input[self.current];
        self.current += 1;
        self.column += 1;
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.current + 1]
        }
    }
    
    fn previous(&self) -> char {
        self.input[self.current - 1]
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    
    fn make_token(&self, token_type: TokenType, lexeme: &str, column: usize) -> Token {
        Token::new(token_type, lexeme.to_string(), self.line, column)
    }
}