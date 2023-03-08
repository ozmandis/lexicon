#[cfg(test)]
use crate::{expression::*, recognizer::*};

#[test]
fn t0() {
    let e0 = Expression::satisfy(|c| c.is_ascii_alphabetic()).star();
    let e1 = Expression::satisfy(|c| c.is_ascii_digit()).star();
    let input = "1223asfsdfsd2dasd12331";
    let res = Recognizer::new(vec![e0, e1])
        .find(input.as_bytes())
        .unwrap();
    println!("{:?}", &input[res.1]);
}
