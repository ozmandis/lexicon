use crate::recognizer;
use std::{collections::BTreeSet, rc::Rc};

pub struct Terminal {
    pub satisfy: Rc<dyn Fn(u8) -> bool>,
    pub follow: BTreeSet<usize>,
}

impl Terminal {
    pub fn new(satisfy: Rc<dyn Fn(u8) -> bool>, follow: BTreeSet<usize>) -> Self {
        Terminal { satisfy, follow }
    }
}

pub struct Manifest {
    pub is_nullable: bool,
    pub first: BTreeSet<usize>,
    pub last: BTreeSet<usize>,
}

impl Manifest {
    pub fn new(is_nullable: bool, first: BTreeSet<usize>, last: BTreeSet<usize>) -> Self {
        Manifest {
            is_nullable,
            first,
            last,
        }
    }
}

pub fn compute_manifest(
    recognizer: &recognizer::Inner,
    position: &mut usize,
    terminals: &mut Vec<Terminal>,
) -> Manifest {
    match recognizer {
        recognizer::Inner::Satisfy(test) => {
            // Get position
            let local = *position;
            *position += 1;
            // Push new corresponding terminal
            terminals.push(Terminal::new(test.clone(), BTreeSet::new()));
            // Result
            Manifest::new(false, BTreeSet::from([local]), BTreeSet::from([local]))
        }
        recognizer::Inner::Max(child) => {
            // Compute manifest of child
            let child = compute_manifest(child, position, terminals);
            // Update follow sets
            for &position in &child.last {
                terminals[position].follow.extend(child.first.clone())
            }
            // Result
            Manifest::new(true, child.first, child.last)
        }
        recognizer::Inner::And(left, right) => {
            // Compute manifest of children
            let left = compute_manifest(left, position, terminals);
            let right = compute_manifest(right, position, terminals);
            // Update follow sets
            for &i in &left.last {
                terminals[i].follow.extend(right.first.clone())
            }
            // Compute first set
            let mut first = left.first;
            if left.is_nullable {
                first.extend(right.first)
            }
            // Compute last set
            let mut last = right.last;
            if right.is_nullable {
                last.extend(left.last)
            }
            // Result
            Manifest::new(left.is_nullable && right.is_nullable, first, last)
        }
        recognizer::Inner::Or(left, right) => {
            // Compute manifest of children
            let left = compute_manifest(left, position, terminals);
            let right = compute_manifest(right, position, terminals);
            // Compute first set
            let mut first = left.first;
            first.extend(right.first);
            // Compute last set
            let mut last = left.last;
            last.extend(right.last);
            // Result
            Manifest::new(left.is_nullable || right.is_nullable, first, last)
        }
    }
}