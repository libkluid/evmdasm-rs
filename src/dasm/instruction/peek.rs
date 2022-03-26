pub(crate) struct Peek<T> {
    pub(crate) data: T,
    pub(crate) size: usize,
}

impl<T> Peek<T> {
    pub(crate) fn new(data: T, size: usize) -> Self {
        Peek { data, size }
    }
}
