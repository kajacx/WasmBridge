use std::marker::PhantomData;

pub struct Linker<T> {
    _phantom: PhantomData<T>,
}
