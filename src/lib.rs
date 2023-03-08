#![allow(dead_code)]

mod test;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    rc::Rc,
};

const STATE_SIZE: usize = u8::MAX as usize + 1;

type PositionSet = BTreeSet<usize>;

#[derive(Clone)]
struct Terminal {
    satisfy: Rc<dyn Fn(u8) -> bool>,
    follow_set: PositionSet,
}

impl Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.follow_set)
    }
}

#[derive(Clone, Debug)]
struct Expression {
    is_nullable: bool,
    first_set: PositionSet,
    last_set: PositionSet,
    terminals: Vec<Terminal>,
}

impl Expression {
    fn satisfy<F: Fn(u8) -> bool + 'static>(satisfy: F) -> Self {
        let terminal = Terminal {
            satisfy: Rc::new(satisfy),
            follow_set: PositionSet::new(),
        };
        Expression {
            is_nullable: false,
            first_set: PositionSet::from([0]),
            last_set: PositionSet::from([0]),
            terminals: Vec::from([terminal]),
        }
    }

    fn star(mut self) -> Self {
        for &position in self.last_set.iter() {
            self.terminals[position]
                .follow_set
                .extend(self.first_set.clone())
        }
        self.is_nullable = true;
        self
    }

    fn and(self, mut other: Self) -> Self {
        //
        other.update_positions(self.terminals.len());
        //
        let mut terminals = self.terminals;
        terminals.extend(other.terminals);
        //
        for &position in self.last_set.iter() {
            terminals[position]
                .follow_set
                .extend(other.first_set.clone());
        }
        //
        let mut first_set = self.first_set;
        if self.is_nullable {
            first_set.extend(other.first_set)
        }
        //
        let mut last_set = other.last_set;
        if other.is_nullable {
            last_set.extend(self.last_set)
        }
        //
        Expression {
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

#[derive(Debug)]
struct Recognizer {
    nullable: Option<usize>,
    dfa: Vec<[Action; STATE_SIZE]>,
}

impl Recognizer {
    fn new(mut expressions: Vec<Expression>) -> Self {
        //
        let root = expressions
            .clone()
            .into_iter()
            .reduce(|l, r| l.or(r))
            .unwrap();
        //
        let mut offset = 0;
        for expression in expressions.iter_mut() {
            expression.update_positions(offset);
            offset += expression.terminals.len();
        }
        //
        let mut nfa = BTreeMap::new();
        nfa_init(&mut nfa, &root.terminals, &expressions, root.first_set);
        //
        let mut correspondance = BTreeMap::new();
        for (i, (k, _)) in nfa.iter().enumerate() {
            correspondance.insert(k.clone(), i);
        }
        //
        let mut dfa = Vec::new();
        for (_, state) in nfa {
            let mut table = [Action::Failure; STATE_SIZE];
            for (i, mut cell) in state.into_iter().enumerate() {
                table[i] = match (cell.0.pop_first(), correspondance.get(&cell.1).copied()) {
                    (None, Some(j)) => Action::GoTo(j),
                    (Some(i), None) => Action::Accept(i),
                    (Some(i), Some(j)) => Action::AcceptAndGoto(i, j),
                    _ => Action::Failure,
                }
            }
            dfa.push(table)
        }
        //
        let nullable = expressions
            .iter()
            .enumerate()
            .find(|(_, e)| e.is_nullable)
            .map(|(i, _)| i);
        Recognizer { nullable, dfa }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    GoTo(usize),
    Accept(usize),
    AcceptAndGoto(usize, usize),
    Failure,
}

fn nfa_init(
    nfa: &mut BTreeMap<PositionSet, Vec<(PositionSet, PositionSet)>>,
    terminals: &Vec<Terminal>,
    expressions: &Vec<Expression>,
    state: PositionSet,
) {
    if !state.is_empty() && !nfa.contains_key(&state) {
        //
        let mut table = vec![(PositionSet::new(), PositionSet::new()); STATE_SIZE];
        for &position in state.iter() {
            let last = last(position, expressions);
            for i in 0..STATE_SIZE {
                if (terminals[position].satisfy)(i as u8) {
                    if let Some(j) = last {
                        table[i].0.insert(j);
                    }
                    table[i].1.extend(terminals[position].follow_set.clone());
                }
            }
        }
        //
        nfa.insert(state, table.clone());
        //
        for follow in table {
            nfa_init(nfa, terminals, expressions, follow.1);
        }
    }
}

fn last(position: usize, expressions: &Vec<Expression>) -> Option<usize> {
    for (i, expression) in expressions.into_iter().enumerate() {
        if expression.last_set.contains(&position) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t() {
        println!(
            "{:?}",
            Recognizer::new(vec![
                Expression::satisfy(|c| c.is_ascii_digit()),
                Expression::satisfy(|c| c.is_ascii_digit()).star()
            ])
        );
    }
}
