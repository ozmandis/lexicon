use std::{collections::BTreeSet, rc::Rc};

pub(crate) type PositionSet = BTreeSet<usize>;

#[derive(Clone)]
pub(crate) struct Terminal {
    pub(crate) satisfy: Rc<dyn Fn(u8) -> bool>,
    pub(crate) follow_set: PositionSet,
}

impl std::fmt::Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.follow_set)
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub(crate) is_nullable: bool,
    pub(crate) first_set: PositionSet,
    pub(crate) last_set: PositionSet,
    pub(crate) terminals: Vec<Terminal>,
}

impl Expression {
    pub fn satisfy<F: Fn(u8) -> bool + 'static>(satisfy: F) -> Self {
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

    pub fn star(mut self) -> Self {
        for &position in self.last_set.iter() {
            self.terminals[position]
                .follow_set
                .extend(self.first_set.clone())
        }
        self.is_nullable = true;
        self
    }

    pub fn and(self, mut other: Self) -> Self {
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

    pub fn or(mut self, mut other: Self) -> Self {
        other.update_positions(self.terminals.len());
        self.terminals.extend(other.terminals);
        self.first_set.extend(other.first_set);
        self.last_set.extend(other.last_set);
        self.is_nullable |= other.is_nullable;
        self
    }

    pub(crate) fn update_positions(&mut self, offset: usize) {
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
