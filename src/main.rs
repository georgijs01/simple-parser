use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {

    let mut t = Tokenizer::init(String::from("abc abc"));
    while let Some(token) = t.next() {
        println!("{}", "token");
    }
}
