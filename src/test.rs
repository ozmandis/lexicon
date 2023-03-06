#![cfg(test)]

use crate::{automaton, Recognizer};

#[test]
fn t0() {
    let r0 = Recognizer::satisfy(|_| true);
    let r1 = r0.clone().and(r0);
    let v = vec![r1.clone(); 4];
    println!("{:?}", automaton::non_finite_automaton_from(v));
}
