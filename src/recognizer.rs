use std::rc::Rc;

pub enum Inner {
    Satisfy(Rc<dyn Fn(u8) -> bool>),
    Max(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
}

pub struct Recognizer(pub Inner);

impl Recognizer {
    pub fn satisfy_from(test: Rc<dyn Fn(u8) -> bool>) -> Self {
        Recognizer(Inner::Satisfy(test))
    }

    pub fn satisfy<F: Fn(u8) -> bool + 'static>(test: F) -> Self {
        Recognizer(Inner::Satisfy(Rc::new(test)))
    }

    pub fn max(self) -> Self {
        Recognizer(Inner::Max(Box::new(self.0)))
    }

    pub fn and(self, other: Self) -> Self {
        Recognizer(Inner::And(Box::new(self.0), Box::new(other.0)))
    }

    pub fn or(self, other: Self) -> Self {
        Recognizer(Inner::Or(Box::new(self.0), Box::new(other.0)))
    }
}
