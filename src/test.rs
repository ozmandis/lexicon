#![cfg(test)]

use crate::{non_finite_automaton::non_finite_automaton_from, recognizer::Recognizer};

#[test]
fn t0() {
    let r = Recognizer::satisfy(|c| c.is_ascii_alphabetic())
        .and(Recognizer::satisfy(|c| c.is_ascii_digit()).max())
        .max();
    println!("{:?}", non_finite_automaton_from(&r));
}
