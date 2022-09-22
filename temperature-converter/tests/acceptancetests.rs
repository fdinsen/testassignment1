extern crate temperature_converter;
use cucumber::{given, then, when, World};
use float_cmp::approx_eq;
use temperature_converter::converter::{c_to_f, f_to_c};

#[derive(World, Debug, Default)]
pub struct State {
    input: Option<f32>,
    output: Option<f32>,
}

fn main() {
    futures::executor::block_on(State::run("features/"));
}

#[given(expr = "we want to know what {float}F is in celsius")]
fn f_to_c_given(w: &mut State, temp: f32) {
    w.input = Some(temp);
}
#[when(expr = "we input it in the f-converter")]
fn f_to_c_when(w: &mut State) {
    match w.input {
        Some(v) => w.output = Some(f_to_c(v)),
        None => assert!(false),
    };
}
#[then(expr = "we get {float}C")]
fn f_to_c_then(w: &mut State, expected: f32) {
    match w.output {
        Some(v) => assert!(approx_eq!(f32, v, expected, epsilon = 0.001)),
        None => assert!(false),
    };
}

#[given(expr = "we want to know what {float}C is in fahrenheit")]
fn c_to_f_given(w: &mut State, temp: f32) {
    w.input = Some(temp);
}
#[when(expr = "we input it in the c-converter")]
fn c_to_f_when(w: &mut State) {
    match w.input {
        Some(v) => w.output = Some(c_to_f(v)),
        None => assert!(false),
    };
}
#[then(expr = "we get {float}F")]
fn c_to_f_then(w: &mut State, expected: f32) {
    match w.output {
        Some(v) => assert!(approx_eq!(f32, v, expected, epsilon = 0.001)),
        None => assert!(false),
    };
}