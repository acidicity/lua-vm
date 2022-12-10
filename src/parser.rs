use std::iter::Peekable;

use crate::{token, ast, lexer};
use lexer::Lexer;
use ast::Program;
use token::Token;

pub struct Parser<'a> {
    iter: Peekable<Lexer<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            iter: lexer.peekable()
        }
    }
}