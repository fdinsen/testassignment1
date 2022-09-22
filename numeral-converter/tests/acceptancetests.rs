extern crate numeral_converter;
use cucumber::{given, then, when, World};
use numeral_converter::numeral_converter::arabic_to_roman;

#[derive(World, Debug, Default)]
pub struct State {
    input: Option<i32>,
    output: Option<String>,
}

fn main() {
    futures::executor::block_on(State::run("features/"));
}

#[given(expr = "we want the roman representation of {int}")]
fn f_to_c_given(w: &mut State, temp: i32) {
    w.input = Some(temp);
}
#[when(expr = "we input it in the converter")]
fn f_to_c_when(w: &mut State) {
    match w.input {
        Some(v) => w.output = Some(arabic_to_roman(&v)),
        None => assert!(false),
    };
}
#[then(expr = "we get {word}")]
fn f_to_c_then(w: &mut State, expected: String) {
    match &w.output {
        Some(v) => assert_eq!(v.to_owned(), expected),
        None => assert!(false),
    };
}
