#![allow(dead_code)]

use std::{collections::BTreeSet, rc::Rc};

mod test;

const STATE_SIZE: usize = u8::MAX as usize + 1;

type PositionSet = BTreeSet<usize>;

type Satisfier = Rc<dyn Fn(u8) -> bool>;

#[derive(Clone)]
struct Terminal {
    satisfy: Satisfier,
    follow_set: PositionSet,
}

#[derive(Clone)]
struct Recognizer {
    is_nullable: bool,
    first_set: PositionSet,
    last_set: PositionSet,
    terminals: Vec<Terminal>,
}

impl Recognizer {
    fn new(
        is_nullable: bool,
        first_set: PositionSet,
        last_set: PositionSet,
        terminals: Vec<Terminal>,
    ) -> Self {
        Recognizer {
            is_nullable,
            first_set,
            last_set,
            terminals,
        }
    }

    fn satisfy<F: Fn(u8) -> bool + 'static>(satisfy: F) -> Self {
        let terminal = Terminal {
            satisfy: Rc::new(satisfy),
            follow_set: PositionSet::new(),
        };
        Recognizer::new(
            false,
            PositionSet::from([0]),
            PositionSet::from([0]),
            Vec::from([terminal]),
        )
    }

    fn star(mut self) -> Self {
        for &position in self.last_set.iter() {
            self.terminals[position]
                .follow_set
                .extend(self.first_set.clone())
        }
        self
    }

    fn and(self, mut other: Self) -> Self {
        other.update_positions(self.terminals.len());
        let mut terminals = self.terminals;
        terminals.extend(other.terminals);
        for &position in self.last_set.iter() {
            terminals[position]
                .follow_set
                .extend(other.first_set.clone());
        }
        let mut first_set = self.first_set;
        if self.is_nullable {
            first_set.extend(other.first_set)
        }
        let mut last_set = other.last_set;
        if other.is_nullable {
            last_set.extend(self.last_set)
        }
        Recognizer {
            is_nullable: self.is_nullable && other.is_nullable,
            first_set,
            last_set,
            terminals,
        }
    }

    fn or(mut self, mut other: Self) -> Self {
        other.update_positions(self.terminals.len());
        self.terminals.extend(other.terminals);
        self.first_set.extend(other.first_set);
        self.last_set.extend(other.last_set);
        self.is_nullable |= other.is_nullable;
        self
    }

    fn update_positions(&mut self, offset: usize) {
        update_position_set(&mut self.first_set, offset);
        update_position_set(&mut self.last_set, offset);
        for terminal in self.terminals.iter_mut() {
            update_position_set(&mut terminal.follow_set, offset);
        }
    }
}

fn update_position_set(set: &mut PositionSet, offset: usize) {
    *set = set.iter().map(|position| position + offset).collect();
}

#[cfg(test)]
mod tests {}
