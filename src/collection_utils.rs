use im_rc::vector::*;

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
