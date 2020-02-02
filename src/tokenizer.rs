use crate::tokenizer::Func::{Sine, Cosine, Tangent, Exp};
use crate::tokenizer::Token::{Var, Const, OpenBrace, CloseBrace, Assign};

pub struct Tokenizer {

    data: Vec<char>,
    current: usize,

}

impl Tokenizer {

    pub fn init(base: String) -> Self {
        Tokenizer { data: base.chars().collect(), current: -1 }
    }

    pub fn next(&mut self) -> Option<Token> {
        // first try to match any single character values
        match self.data[self.current] {
            '(' => return Some(OpenBrace),
            ')' => return Some(CloseBrace),
            '=' => return Some(Assign),
            _ => (),
        }

        let mut state = State::Undef;

        loop {

        }
    }

}

pub enum State {
    Undef,
    Name,
    Const,

}


pub enum Token {

    Var(String),
    Const(i64),
    Func(Func), // includes an opening brace after it, i.e. must be closed by a CloseBrace
    OpenBrace,
    CloseBrace,
    Assign,
    Op(Op),

}

pub enum Func {

    Sine,
    Cosine,
    Tangent,
    Exp,

}

pub enum Op {

    Add,
    Sub,
    Mult,
    Div,
    Pow
}