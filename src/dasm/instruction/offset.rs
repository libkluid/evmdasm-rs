use std::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Offset<T> {
    pub offset: usize,
    pub data: T,
}

impl<T> Deref for Offset<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
