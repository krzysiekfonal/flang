#[derive(Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Equal,
    Less, LessEq, Greater, GreaterEq, 

    EOF
}

#[derive(Debug)]
pub enum Literal {
    NumericInt(i32),
    NumericFloat(f32),
    Text(String),
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenType,
    lexeme: &'a str,
    literal: Option<Literal>,
    line: usize,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn from_script(source: &'a str) -> Self {
        Self { source, start: 0, current: 0, line: 1 }
    }

    pub fn scan(&mut self) -> Vec<Token<'a>> {
        let mut tokens: Vec<Token<'a>> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(token) => {
                    println!("Found token {:?}", token);
                    tokens.push(token);
                }
                Err(e) => println!("Syntax error found {}", e),
            }
        }
        tokens.push(Token{
            kind: TokenType::EOF,
            lexeme: "",
            literal: None,
            line: self.line,
        });

        tokens
    }

    fn scan_token(&mut self) -> Result<Token<'a>, String> {
        let c = self.advance();
        match c {
            Some('(') => Ok(self.gen_token(TokenType::LeftParen, None)),
            Some(')') => Ok(self.gen_token(TokenType::RightParen, None)),
            Some('{') => Ok(self.gen_token(TokenType::LeftBrace, None)),
            Some('}') => Ok(self.gen_token(TokenType::RightBrace, None)),
            Some(',') => Ok(self.gen_token(TokenType::Comma, None)),
            Some('.') => Ok(self.gen_token(TokenType::Dot, None)),
            Some(';') => Ok(self.gen_token(TokenType::Semicolon, None)),
            Some('/') => Ok(self.gen_token(TokenType::Slash, None)),
            Some('-') => Ok(self.gen_token(TokenType::Minus, None)),
            Some('+') => Ok(self.gen_token(TokenType::Plus, None)),
            Some('*') => Ok(self.gen_token(TokenType::Star, None)),

            Some('<') => {
                let token_type = if self.match_next('=') {TokenType::LessEq} else {TokenType::Less};
                Ok(self.gen_token(token_type, None))
            }
            Some('>') => {
                let token_type = if self.match_next('=') {TokenType::GreaterEq} else {TokenType::Greater};
                Ok(self.gen_token(token_type, None))
            }
 
            Some(ch) => Err(format!("Unkown character {ch}")),

            None => Err(format!("Unexpected end of stream/file")),
        }
    }

    fn gen_token(&mut self, kind: TokenType, literal: Option<Literal>) -> Token<'a> {
        Token{
            kind: kind, 
            lexeme: &self.source[self.start..self.current], 
            literal: literal,
            line: self.line
        }
    }

    fn is_at_end(&mut self) -> bool {
        !(self.current < self.source.len())
    }

    fn advance(&mut self) -> Option<char> {
        let c: char = self.source[self.current..].chars().next()?;
        self.current += c.len_utf8();
        Some(c)
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        let c = self.advance();
        match (c) {
            Some(ch) => if ch == expected {return true} else {self.current -= ch.len_utf8(); false}
            None => false
        }

    }
}
