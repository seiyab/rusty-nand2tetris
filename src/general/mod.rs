pub type T2<T> = (T, T);
pub type T4<T> = T2<T2<T>>;
pub type T8<T> = T2<T4<T>>;
pub type T16<T> = T4<T4<T>>;
