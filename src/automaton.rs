use std::collections::{BTreeMap, BTreeSet};

use crate::{manifest::*, Recognizer};

type NonFiniteAutomaton = BTreeMap<BTreeSet<usize>, Vec<BTreeSet<usize>>>;

pub fn non_finite_automaton_from(recognizers: Vec<Recognizer>) -> NonFiniteAutomaton {
    let position = &mut 0;
    let terminals = &mut Vec::new();
    let manifests: Vec<Manifest> = recognizers
        .iter()
        .map(|r| compute_manifest(&r.0, position, terminals))
        .collect();
    let first = manifests.iter().fold(BTreeSet::new(), |mut acc, x| {
        acc.extend(x.first.clone());
        acc
    });
    let mut automaton = BTreeMap::new();
    non_finite_automaton_init(&mut automaton, terminals, first);
    automaton
}

fn non_finite_automaton_init(
    automaton: &mut NonFiniteAutomaton,
    terminals: &Vec<Terminal>,
    state: BTreeSet<usize>,
) {
    if !state.is_empty() && !automaton.contains_key(&state) {
        let mut table = vec![BTreeSet::new(); u8::MAX as usize + 1];
        for &position in state.iter() {
            for i in 0..=u8::MAX {
                if (terminals[position].satisfy)(i) {
                    table[i as usize].extend(terminals[position].follow.clone());
                }
            }
        }
        for follow in &table {
            non_finite_automaton_init(automaton, terminals, follow.clone());
        }
        automaton.insert(state, table);
    }
}
