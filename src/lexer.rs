use std::{iter::Peekable, str::CharIndices};
use crate::token::{Token, lookup_identifier};

pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let tok: Token = self.next_token();

        if tok != Token::EOF {
            Some(tok)
        } else {
            None
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable() 
        }
    }

    fn read_char(&mut self) -> Option<(usize, char)>{
        self.iter.next()
    }

    pub fn next_token(&mut self) -> Token<'a>{
        
            self.skip_whitespace();

            let current: Option<(usize, char)> = self.read_char();

            match current {
                Some((_, '=')) => {
                    if self.iter.peek().unwrap().1 == '=' {
                        self.read_char();
                        Token::Equal
                    } else {
                        Token::Assign
                    }
                },
                Some((_, '+')) => Token::Plus,
                Some((_, '-')) => Token::Minus, 
                Some((_, '!')) => {
                    if self.iter.peek().unwrap().1 == '=' {
                        self.read_char();
                        Token::NotEqual
                    } else {
                        Token::Bang
                    }
                }, 
                Some((_, '*')) => Token::Asterisk, 
                Some((_, '/')) => Token::Slash, 
                Some((_, '<')) => Token::LessThan, 
                Some((_, '>')) => Token::GreaterThan, 
                Some((_, ',')) => Token::Comma,
                Some((_, ';')) => Token::Semicolon,
                Some((_, '(')) => Token::LeftParenthesis,
                Some((_, ')')) => Token::RightParenthesis,
                Some((_, '{')) => Token::LeftBrace,
                Some((_, '}')) => Token::RightBrace,
                Some((_, ch)) => {
                    if is_letter(ch) {
                        let literal = self.read_identifier(current.unwrap().0);
                        lookup_identifier(literal)
                    } else if is_digit(ch) {
                        Token::Integer(self.read_number(current.unwrap().0))
                    } else {
                        Token::Illegal
                    }
                }
                None => Token::EOF
            }
    }

    fn read_identifier(&mut self, start: usize) -> &'a str{
        let start_pos: usize = start;
        let mut end_pos: usize = start_pos + 1;

        while is_letter(self.iter.peek().unwrap().1) {
            self.read_char();
            end_pos += 1;
        }

        &self.input[start_pos..end_pos]
    }

    fn read_number(&mut self, start: usize) -> i32{
        let start_pos: usize = start;
        let mut end_pos: usize = start_pos + 1;

        while is_digit(self.iter.peek().unwrap().1) {
            self.read_char();
            end_pos += 1;
        }

        self.input[start_pos..end_pos].parse::<i32>().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.iter.peek() {
            if !ch.1.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric() || ch == '_'
}
