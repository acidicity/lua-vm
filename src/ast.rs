pub struct Program<'a> {
    statements: Vec<Statement<'a>>
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self{
            statements: Vec::new()
        }
    }
}

pub enum Statement<'a> {
    Let(&'a str, Expression<'a>)
}

pub enum Expression<'a> {
    Identifier(&'a str)
}