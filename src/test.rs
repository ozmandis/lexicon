#[cfg(test)]
use crate::{expression::*, recognizer::*};

#[test]
fn benchmark() {
    let input = vec!['a' as u8; 1_000_000_000];
    let start = std::time::Instant::now();
    let f = Recognizer::new(vec![
        Expression::satisfy(|c| c == 'a' as u8).star(),
        Expression::satisfy(|c| c == 'b' as u8),
    ])
    .find(input.as_slice())
    .unwrap();
    println!("{:?}\n{:?}", f, start.elapsed());
}
