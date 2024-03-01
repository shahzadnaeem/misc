use std::vec;

pub fn new_vec<T>(sz: usize) -> Vec<T>
where
    T: Clone + Default,
{
    vec![T::default(); sz]
}
