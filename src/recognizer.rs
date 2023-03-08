use std::collections::BTreeMap;

use crate::expression::*;

const STATE_SIZE: usize = u8::MAX as usize + 1;

#[derive(Debug)]
pub struct Recognizer {
    nullable: Option<usize>,
    dfa: Vec<[Action; STATE_SIZE]>,
}

impl Recognizer {
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
