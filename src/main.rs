use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {

    let mut t = Tokenizer::init(String::from("sin(a + b) = n"));
    while let Some(token) = t.next() {
        println!("{:?}", token);
    }
}
