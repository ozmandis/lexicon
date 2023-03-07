use std::collections::{BTreeMap, BTreeSet};

use crate::{manifest::*, recognizer};

type NonFiniteAutomaton = BTreeMap<BTreeSet<usize>, Vec<BTreeSet<usize>>>;

fn non_finite_automaton_from(
    recognizers: &Vec<recognizer::Inner>,
) -> (NonFiniteAutomaton, Vec<Manifest>) {
    let position = &mut 0;
    let terminals = &mut Vec::new();
    let manifests: Vec<Manifest> = recognizers
        .iter()
        .map(|r| compute_manifest(&r, position, terminals))
        .collect();
    let mut first = BTreeSet::new();
    for manifest in manifests.iter() {
        first.extend(manifest.first.clone());
    }

    let mut automaton = BTreeMap::new();
    non_finite_automaton_init(&mut automaton, terminals, first);
    (automaton, manifests)
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
        automaton.insert(state, table.clone());
        for follow in table {
            non_finite_automaton_init(automaton, terminals, follow);
        }
    }
}

pub(crate) fn deterministic_finite_automata(
    recognizers: &Vec<recognizer::Inner>,
) -> Vec<Vec<Option<usize>>> {
    let (nfa, manifests) = non_finite_automaton_from(recognizers);
    let mut correspondance = BTreeMap::new();
    for (i, (k, _)) in nfa.iter().enumerate() {
        correspondance.insert(k, i);
    }
    let mut dfa = Vec::new();
    for (_, v) in nfa.iter() {
        let mut table = Vec::new();
        for i in v {
            table.push(if i.is_empty() {
                None
            } else {
                Some(correspondance[i])
            })
        }
        dfa.push(table)
    }
    dfa
}
