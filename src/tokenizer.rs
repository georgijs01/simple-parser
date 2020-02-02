use crate::tokenizer::Token::{Var, Const, OpenBrace, CloseBrace, Assign, Func, Op};
use crate::tokenizer::FuncType::{Sine, Cosine, Tangent, Exp};
use crate::tokenizer::OpType::{Sub, Add, Mult, Div, Pow};

pub struct Tokenizer {

    data: Vec<char>,
    current: usize,

}

impl Tokenizer {

    pub fn init(base: String) -> Self {
        Tokenizer { data: base.chars().collect(), current: 0 }
    }

    pub fn next(&mut self) -> Option<Token> {

        enum State {
            Undef,
            Name,
            Const,
        }

        // skip whitespace
        while self.current < self.data.len() && self.data[self.current as usize] == ' ' {
            self.current += 1;
        }

        // if the data ends with whitespace, return None
        if self.current >= self.data.len() {
            return None
        }

        // first try to match any single character values
        self.current += 1;
        match self.data[self.current - 1] {
            '(' => return Some(OpenBrace),
            ')' => return Some(CloseBrace),
            '=' => return Some(Assign),
            '+' => return Some(Op(Add)),
            '*' => return Some(Op(Mult)),
            '/' => return Some(Op(Div)),
            '^' => return Some(Op(Pow)),
            _ => self.current -= 1,
        }

        let mut state = State::Undef;
        let start = self.current as usize;
        let mut read = String::new();

        while self.current < self.data.len() {
            match (&mut state, self.data[self.current]) {
                (State::Undef, '0' ..= '9') => state = State::Const,
                (State::Undef, x @ _) => {
                    if x.is_alphabetic() {
                        state = State::Name;
                    } else if x == '-' {
                        if self.current + 1 < self.data.len()
                            && self.data[self.current + 1].is_digit(10) {
                            state = State::Const;
                        } else {
                            self.current += 1;
                            return Some(Op(Sub));
                        }
                    } else {
                        return None;
                    }
                }
                (State::Name, '(') => {
                    self.current += 1;
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
                        return Some(Var(read));
                    }
                }
                (State::Const, x @ _) => {
                    if !x.is_digit(10) {
                        return Some(Const(read.parse::<i64>().unwrap()))
                    }
                }
                _ => (),
            }
            read.push(self.data[self.current as usize]);
            self.current += 1;
        }

        return match &mut state {
            State::Undef => None,
            State::Name => Some(Var(read)),
            State::Const => Some(Const(read.parse::<i64>().unwrap())),
        }
    }
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
//
//#[cfg(test)]
//mod test {
//
//    use super::*;
//
//    #[test]
//    fn test_basics() {
//        let mut t = Tokenizer::init(String::from("n = sin(a + b) * c"));
//    }
//
//}






















