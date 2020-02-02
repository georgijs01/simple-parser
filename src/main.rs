use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    let mut t = Tokenizer::init(String::from("sin(abc^b)=n+-3+(7*-5)"));
    while let Some(token) = t.next() {
        println!("{:?}", token);
    }
}
