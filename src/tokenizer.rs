use crate::tokenizer::Token::{Var, Const, OpenBrace, CloseBrace, Assign, Func, Op};
use crate::tokenizer::FuncType::{Sine, Cosine, Tangent, Exp};
use crate::tokenizer::OpType::{Sub, Add, Mult, Div};

pub struct Tokenizer {

    data: Vec<char>,
    current: isize,

}

impl Tokenizer {

    pub fn init(base: String) -> Self {
        Tokenizer { data: base.chars().collect(), current: -1 }
    }

    pub fn next(&mut self) -> Option<Token> {

        // increment the current counter, as this is not done before this method returns
        self.current += 1;

        // skip whitespace
        while self.current < self.data.len() as isize && self.data[self.current as usize] == ' ' {
            self.current += 1;
        }

        // if the data ends with whitespace, return None
        if self.current >= self.data.len() as isize {
            return None
        }

        // first try to match any single character values
        match self.data[self.current as usize] {
            '(' => return Some(OpenBrace),
            ')' => return Some(CloseBrace),
            '=' => return Some(Assign),
            _ => (),
        }

        let mut state = State::Undef;
        let start = self.current as usize;
        let mut read = String::new();

        while self.current < self.data.len() as isize {
            match (&mut state, self.data[self.current as usize]) {
                (State::Undef, '0' ..= '9') => state = State::Const,
                (State::Undef, 'A' ..= 'z') => state = State::Name,
                (State::Undef, x @ _) => {
                    match x {
                        '+' => return Some(Op(Add)),
                        '*' => return Some(Op(Mult)),
                        '/' => return Some(Op(Div)),
                        '-' => {
                            if self.current + 1 < self.data.len() as isize
                                && self.data[self.current as usize + 1].is_digit(10) {
                                state = State::Const;
                            } else {
                                return Some(Op(Sub))
                            }
                        },
                        _ => return None,
                    }
                }
                (State::Name, '(') => {
                    return match read.as_str() {
                        "sin" => Some(Func(Sine)),
                        "cos" => Some(Func(Cosine)),
                        "tan" => Some(Func(Tangent)),
                        "exp" => Some(Func(Exp)),
                        _ => None,
                    }
                },
                (State::Name, x @ _) => {
                    if !x.is_alphabetic() {
                        self.current -= 1;
                        return Some(Var(read));
                    }
                }
                (State::Const, x @ _) => {
                    if !x.is_digit(10) {
                        self.current -= 1;
                        return Some(Const(read.parse::<i64>().unwrap()))
                    }
                }
                _ => (),
            }
            read.push(self.data[self.current as usize]);
            self.current += 1;
        }

        match &mut state {
            State::Undef => return None,
            State::Name => return Some(Var(read)),
            State::Const => Some(Const(read.parse::<i64>().unwrap())),
        }
    }
}

pub enum State {
    Undef,
    Name,
    Const,

}

#[derive(Debug)]
pub enum Token {

    Var(String),
    Const(i64),
    Func(FuncType), // includes an opening brace after it, i.e. must be closed by a CloseBrace
    OpenBrace,
    CloseBrace,
    Assign,
    Op(OpType),

}

#[derive(Debug)]
pub enum FuncType {

    Sine,
    Cosine,
    Tangent,
    Exp,

}

#[derive(Debug)]
pub enum OpType {

    Add,
    Sub,
    Mult,
    Div,
    Pow
}