pub trait Zero {
    fn new() -> Self;
}

impl<T: Zero> Zero for Box<T> {
    fn new() -> Self {
        Box::new(T::new())
    }
}
