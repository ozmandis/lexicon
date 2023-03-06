use crate::{manifest::compute_manifest, recognizer::*};
use std::{
    collections::{BTreeMap, BTreeSet},
    vec,
};

#[derive(Clone, Debug)]
pub enum Action {
    Continue(BTreeSet<usize>),
    AcceptAndNext(BTreeSet<usize>),
    Accept,
    Fail,
}

impl Action {
    fn extend(&mut self, other: BTreeSet<usize>) {
        match self {
            Self::Continue(set) => set.extend(other),
            Self::AcceptAndNext(set) => set.extend(other),
            _ => *self = Self::Continue(other),
        }
    }
    fn set(&self) -> BTreeSet<usize> {
        match self {
            Self::Continue(set) => set.clone(),
            _ => BTreeSet::new(),
        }
    }
}

type NonFiniteAutomaton = BTreeMap<BTreeSet<usize>, Vec<Action>>;

pub fn non_finite_automaton_from(recognizer: &Recognizer) -> NonFiniteAutomaton {
    let mut automaton = BTreeMap::new();
    let mut terminals = Vec::new();
    let root = compute_manifest(&recognizer.0, &mut 0, &mut terminals);
    let mut states = BTreeSet::from([root.first.clone()]);
    while !states.is_empty() {
        let mut next_states = BTreeSet::new();
        for state in &states {
            let mut table = vec![Action::Fail; u8::MAX as usize + 1];
            for &position in state {
                for i in 0..=u8::MAX {
                    if (terminals[position].satisfy)(i) {
                        table[i as usize].extend(terminals[position].follow.clone())
                    }
                }
            }
            for action in &table {
                let set = action.set();
                if !set.is_empty() && !automaton.contains_key(&set) {
                    next_states.insert(set);
                }
            }
            automaton.insert(state.clone(), table);
        }
        states = next_states;
    }
    for (k, v) in automaton.iter_mut() {
        let inter: Vec<&usize> = root.last.intersection(k).collect();
        if !inter.is_empty() {
            for x in v {
                match x {
                    Action::Continue(set) => *x = Action::AcceptAndNext(set.clone()),
                    _ => {}
                }
            }
        }
    }
    if root.is_nullable {
        for x in automaton.get_mut(&root.first).unwrap() {
            match x {
                Action::Fail => *x = Action::Accept,
                _ => {}
            }
        }
    }
    automaton
}
