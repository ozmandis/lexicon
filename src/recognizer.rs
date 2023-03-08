use std::{collections::BTreeMap, ops::Range};

use crate::expression::*;

const STATE_SIZE: usize = u8::MAX as usize + 1;

#[derive(Debug)]
pub struct Recognizer {
    first_state: usize,
    nullable: Option<usize>,
    dfa: Vec<[Action; STATE_SIZE]>,
}

impl Recognizer {
    pub fn find(&self, input: &[u8]) -> Option<(usize, Range<usize>)> {
        let mut res = if let Some(nullable) = self.nullable {
            Some((nullable, 0..0))
        } else {
            None
        };
        let mut it = input.iter().enumerate();
        let mut state = self.first_state;
        while let Some((i, &c)) = it.next() {
            match self.dfa[state][c as usize] {
                Action::Failure => break,
                Action::Accept(r) => {
                    res = Some((r, 0..i + 1));
                    break;
                }
                Action::AcceptAndGoto(r, n) => {
                    res = Some((r, 0..i + 1));
                    state = n;
                }
                Action::GoTo(n) => state = n,
            }
        }
        res
    }

    pub fn new(mut expressions: Vec<Expression>) -> Self {
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
        nfa_init(
            &mut nfa,
            &root.terminals,
            &expressions,
            root.first_set.clone(),
        );
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
        Recognizer {
            first_state: correspondance[&root.first_set],
            nullable,
            dfa,
        }
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
            let accept = find_accept(position, expressions);
            for i in 0..STATE_SIZE {
                if (terminals[position].satisfy)(i as u8) {
                    if let Some(j) = accept {
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

fn find_accept(position: usize, expressions: &Vec<Expression>) -> Option<usize> {
    for (i, expression) in expressions.into_iter().enumerate() {
        if expression.last_set.contains(&position) {
            return Some(i);
        }
    }
    None
}
