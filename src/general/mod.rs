pub trait Zero {
    fn new() -> Self;
}

impl<T: Zero> Zero for Box<T> {
    fn new() -> Self {
        Box::new(T::new())
    }
}

impl<T: Zero> Zero for [T; 3] {
    fn new() -> Self {
        [T::new(), T::new(), T::new()]
    }
}

impl<T: Zero> Zero for [T; 16] {
    fn new() -> Self {
        [
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
            T::new(),
        ]
    }
}

impl<A: Zero, B: Zero> Zero for (A, B) {
    fn new() -> Self {
        (A::new(), B::new())
    }
}
