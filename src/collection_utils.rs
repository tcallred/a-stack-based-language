use im_rc::vector::*;
use std::str::Chars;
use std::rc::Rc;

pub trait Collection<T: Clone> {
    fn conj(self, value: T) -> Self;
}

impl<T: Clone> Collection<T> for Vector<T> {
    fn conj(mut self, value: T) -> Self {
        self.push_back(value);
        self
    }
}

impl Collection<char> for String {
    fn conj(mut self, value: char) -> Self {
        self.push(value);
        self
    }
}

#[derive(Clone)]
pub enum List<T: Clone> {
    Empty,
    Cons(T, Rc<List<T>>),
}

impl<T: Clone> List<T> {
    pub fn empty() -> Self {
        List::Empty
    }

    pub fn cons(self, value: T) -> Self {
        List::Cons(value, Rc::new(self))
    }

    pub fn head(self) -> Option<T> {
        match self {
            List::Empty => None,
            List::Cons(value, _) => Some(value),
        }
    }

    pub fn tail(self) -> Self {
        match self {
            List::Empty => List::Empty,
            List::Cons(_, rest) => (*rest).clone(),
        }
    }
}

pub trait Sequence: Iterator + DoubleEndedIterator {
    fn seq(self) -> List<Self::Item>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        self.rev().fold(List::Empty, |lst, value| lst.cons(value))
    }
}

impl<'a> Sequence for Chars<'a> {}
